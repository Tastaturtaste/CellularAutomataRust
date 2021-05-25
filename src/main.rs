mod cell;
mod game;
mod game_board;
mod game_rules;
mod globals;
use globals::OVERFLOW_MSG;

use std::time::{Duration, Instant};

use core::f32;
use std::convert::TryInto;

use cell::*;
use game::*;
use pixels;
use winit::{
    dpi::{self, PhysicalPosition},
    event::{
        ElementState, Event, KeyboardInput, ModifiersState, MouseButton, VirtualKeyCode,
        WindowEvent,
    },
    event_loop::{ControlFlow, EventLoop, EventLoopProxy},
    window::{Window, WindowBuilder},
};

pub struct RGBA([u8; 4]);

impl RGBA {
    pub fn black() -> RGBA {
        RGBA {
            0: [0x00, 0x00, 0x00, 0x00],
        }
    }
    pub fn white() -> RGBA {
        RGBA {
            0: [0xFF, 0xFF, 0xFF, 0xFF],
        }
    }
    pub fn get_raw(&self) -> [u8; 4] {
        self.0
    }
    pub fn r(&mut self) -> &mut u8 {
        &mut self.0[0]
    }
    pub fn g(&mut self) -> &mut u8 {
        &mut self.0[1]
    }
    pub fn b(&mut self) -> &mut u8 {
        &mut self.0[2]
    }
    pub fn a(&mut self) -> &mut u8 {
        &mut self.0[3]
    }
}

impl CellConway {
    pub fn to_rgba(&self) -> RGBA {
        match *self {
            Self::Dead => RGBA::black(),
            Self::Alive => RGBA::white(),
        }
    }
}

pub struct VisualGame {
    game: ConwaysGame,
    pixel_buffer: pixels::Pixels,
    update_time: Duration,
    frame_time: Duration,
    paused: bool,
    cell_size: f32, // float because dpi scaling can be rational
    trail_decay: f32,
}

impl VisualGame {
    pub fn new(width: usize, height: usize, cell_size: f32, window: &Window) -> VisualGame {
        let width = (width as f32 / cell_size as f32).round() as usize;
        let height = (height as f32 / cell_size as f32).round() as usize;

        let game = ConwaysGame::new_rand(width, height);

        let surface = pixels::SurfaceTexture::new(
            width.try_into().expect(OVERFLOW_MSG),
            height.try_into().expect(OVERFLOW_MSG),
            window,
        );
        let mut pixel_buffer = pixels::Pixels::new(
            width.try_into().expect(OVERFLOW_MSG),
            height.try_into().expect(OVERFLOW_MSG),
            surface,
        )
        .expect("Cannot create pixel texture!");
        pixel_buffer.resize_buffer(width as u32, height as u32);

        let update_time = Duration::new(1, 0).div_f32(4.);
        let frame_time = Duration::new(1, 0).div_f32(25.); // Das menschliche Auge kann nur 24 fps sehen. Deshalb 24 fps +1 für die Sicherheit
        let paused = false;
        let trail_decay = 0.0; //0.9;
        VisualGame {
            game,
            pixel_buffer,
            update_time,
            frame_time,
            paused,
            cell_size,
            trail_decay,
        }
    }
    pub fn step(&mut self) {
        if !self.paused {
            self.game.step();
        }
    }
    pub fn step_ignore_pause(&mut self) {
        debug_assert!(
            !self.paused,
            "The step method should only be used in the paused state."
        );
        self.game.step();
    }
    pub fn update_pixel_buffer(&mut self) {
        for (pixel, c) in self
            .pixel_buffer
            .get_frame()
            .chunks_exact_mut(4)
            .zip(self.game.get_board().into_iter())
        {
            if *c == CellConway::Alive {
                pixel.copy_from_slice(&c.to_rgba().get_raw());
            } else {
                let trail_decay = &self.trail_decay;
                pixel
                    .iter_mut()
                    .for_each(|byte| *byte = (*byte as f32 * trail_decay) as u8);
            }
        }
    }
    pub fn render(&mut self) -> Result<(), pixels::Error> {
        self.pixel_buffer.render()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum UserEvent {
    StepCell { x: usize, y: usize },
}

struct MouseState {
    position: PhysicalPosition<f64>,
    left: ElementState,
    right: ElementState,
    middle: ElementState,
}

impl MouseState {
    pub fn update_button(&mut self, input: MouseInput) {
        match input.button {
            MouseButton::Left => self.left = input.state,
            MouseButton::Right => self.right = input.state,
            MouseButton::Middle => self.middle = input.state,
            _ => {}
        }
    }
    pub fn update_position(&mut self, position: PhysicalPosition<f64>) {
        self.position = position;
    }
}

struct MouseInput {
    state: ElementState,
    button: MouseButton,
}

struct GameContext {
    last_cell_stepped: Option<(usize, usize)>,
}

fn main() {
    // let event_loop = EventLoop::new();
    let event_loop = EventLoop::<UserEvent>::with_user_event();
    let event_loop_proxy = event_loop.create_proxy();

    let (window, window_size, dpi_factor) = make_window("Game of Life", &event_loop);
    let cell_size = dpi_factor * 8 as f32;
    let mut vgame = VisualGame::new(
        window_size.width as usize,
        window_size.height as usize,
        cell_size,
        &window,
    );
    let mut last_game_update = std::time::Instant::now();
    let mut last_update_duration = Duration::new(0, 0); // Store the time required per update to enable more accurate frame and update intervals
    let mut modifier_state = ModifiersState::empty();
    let mut mouse_state = MouseState {
        position: PhysicalPosition { x: 0., y: 0. },
        left: ElementState::Released,
        right: ElementState::Released,
        middle: ElementState::Released,
    };
    let mut game_context = GameContext {
        last_cell_stepped: None,
    };

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    println!("The close button was pressed; stopping");
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::Resized(size) => {
                    vgame.pixel_buffer.resize_surface(size.width, size.height);
                    window.request_redraw();
                }
                WindowEvent::ModifiersChanged(state) => {
                    modifier_state = state;
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    on_keyboard_input(input, &modifier_state, &mut vgame, &window);
                }
                WindowEvent::CursorMoved { position, .. } => {
                    mouse_state.update_position(position);
                    on_mouse_state_updated(
                        &mouse_state,
                        &modifier_state,
                        &mut vgame,
                        &mut game_context,
                        &window,
                        &event_loop_proxy,
                    )
                }
                WindowEvent::MouseInput { button, state, .. } => {
                    mouse_state.update_button(MouseInput { button, state });
                    on_mouse_state_updated(
                        &mouse_state,
                        &modifier_state,
                        &mut vgame,
                        &mut game_context,
                        &window,
                        &event_loop_proxy,
                    )
                }
                _ => {}
            },
            Event::UserEvent(user_event) => on_user_event(
                user_event,
                &mut vgame,
                &mut game_context,
                &window,
                &event_loop_proxy,
            ),
            Event::MainEventsCleared => {
                let begin = Instant::now();
                let update_delay = begin - last_game_update;
                if (update_delay + last_update_duration) >= vgame.update_time {
                    vgame.step();
                    last_game_update = Instant::now();
                    last_update_duration = last_game_update - begin;
                    window.request_redraw();
                }
            }
            Event::RedrawRequested(_) => {
                vgame.update_pixel_buffer();
                if vgame.render().is_err() {
                    eprintln!("Error: Could not render to pixel buffer!");
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
            _ => (),
        }
    });
}

// Should not depend on raw input state such as key presses, modifier state or mouse state
// That should already be translated into a specific game event
fn on_user_event(
    event: UserEvent,
    vgame: &mut VisualGame,
    game_context: &mut GameContext,
    window: &Window,
    event_loop_proxy: &EventLoopProxy<UserEvent>,
) {
    match event {
        UserEvent::StepCell { x, y } => {
            if let Some((last_x, last_y)) = game_context.last_cell_stepped {
                if last_x == x && last_y == y {
                    return;
                }
            }
            vgame.game.step_cell(x, y);
            game_context.last_cell_stepped = Some((x, y));
        }
        _ => {}
    }
}

fn on_mouse_state_updated(
    mouse_state: &MouseState,
    modifier_state: &ModifiersState,
    vgame: &mut VisualGame,
    game_context: &mut GameContext,
    window: &Window,
    event_loop_proxy: &EventLoopProxy<UserEvent>,
) {
    let (x, y) = vgame
        .pixel_buffer
        .window_pos_to_pixel(mouse_state.position.into())
        .unwrap_or_else(|pos| vgame.pixel_buffer.clamp_pixel_pos(pos));
    if let ElementState::Pressed = mouse_state.left {
        event_loop_proxy
            .send_event(UserEvent::StepCell { x, y })
            .expect("Sending event to a proxy event loop failed!");
    } else {
        game_context.last_cell_stepped = None;
    }
}

fn on_keyboard_input(
    input: KeyboardInput,
    modifier_state: &ModifiersState,
    game: &mut VisualGame,
    window: &Window,
) {
    let KeyboardInput {
        scancode: _,
        state,
        virtual_keycode,
        //modifiers,
        ..
    } = input;
    if state == ElementState::Released {
        return;
    }
    match virtual_keycode {
        Some(VirtualKeyCode::P) => game.paused = !game.paused,
        Some(VirtualKeyCode::NumpadAdd) => {
            match *modifier_state {
                ModifiersState::CTRL => {
                    game.frame_time = game.frame_time.mul_f32(0.9);
                    println!("Decrease frame_time");
                }
                ModifiersState::SHIFT => {
                    // Since a smaller value implies a higher decay rate we add to make decay faster
                    game.trail_decay += 0.1;
                    game.trail_decay = game.trail_decay.clamp(0.0, 1.0);
                    println!("Increased decay constant to {}", game.trail_decay);
                }
                _ => {
                    game.update_time = game.update_time.mul_f32(0.9);
                    println!("Decreased update_time");
                }
            }
        }
        Some(VirtualKeyCode::NumpadSubtract) => {
            match *modifier_state {
                ModifiersState::CTRL => {
                    game.frame_time = game.frame_time.mul_f32(1.1);
                    println!("Increased frame_time");
                }
                ModifiersState::SHIFT => {
                    // Since a bigger value implies a lower decay rate we subtract to make decay slower
                    game.trail_decay -= 0.1;
                    game.trail_decay = game.trail_decay.clamp(0.0, 1.0);
                    println!("Decreased decay rate to {}", game.trail_decay);
                }
                _ => {
                    game.update_time = game.update_time.mul_f32(1.1);
                    println!("Increased update_time")
                }
            }
        }
        Some(VirtualKeyCode::Space) => {
            game.step_ignore_pause();
            window.request_redraw();
        }
        _ => {}
    }
}

fn make_window<T>(title: &str, event_loop: &EventLoop<T>) -> (Window, dpi::PhysicalSize<u32>, f32) {
    let window = WindowBuilder::new()
        .with_position(PhysicalPosition::new(0, 0))
        .with_visible(false)
        .with_title(title)
        .build(event_loop)
        .expect("Could not construct window!");
    let dpi_factor = window.scale_factor();
    let window_size = dpi::PhysicalSize::new(globals::DEFAULT_SIZE.0, globals::DEFAULT_SIZE.1);
    let monitor_size = if let Some(monitor) = window.current_monitor() {
        monitor.size()
    } else {
        window_size
    };
    // Scale window to 2/3 of monitor size
    let scale = (monitor_size.width as f64 / window_size.width as f64 * 2.0 / 3.0)
        .round()
        .max(1.0);
    let inner_size = dpi::PhysicalSize::new(
        window_size.width as f64 * scale,
        window_size.height as f64 * scale,
    );
    window.set_inner_size(inner_size);
    window.set_min_inner_size(Some(dpi::PhysicalSize::new(
        inner_size.width.floor(),
        inner_size.height.floor(),
    )));
    window.set_visible(true);
    (
        window,
        dpi::PhysicalSize::new(inner_size.width as u32, inner_size.height as u32),
        dpi_factor as f32,
    )
}
