mod cell;
mod game;
mod game_board;
mod game_rules;
mod globals;
use globals::OVERFLOW_MSG;

use std::{
    ops::{Add, Sub},
    time::{self, Duration, Instant},
};

use core::f32;
use std::convert::TryInto;

use cell::*;
use game::*;
use game_board::*;

use pixels;
use winit::{
    dpi::{self, PhysicalPosition, PhysicalSize},
    event::{ElementState, Event, KeyboardInput, ModifiersState, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
// use winit_input_helper::WinitInputHelper;

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
        let trail_decay = 0.9;
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
    pub fn evolve(&mut self) {
        if !self.paused {
            self.game.evolve();
        }
    }
    pub fn step(&mut self) {
        debug_assert!(
            !self.paused,
            "The step method should only be used in the paused state."
        );
        self.game.evolve();
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
    pub fn on_clear(&mut self) {
        (&mut self.game)
            .get_board_mut()
            .into_iter()
            .for_each(|c: &mut CellConway| *c = CellConway::Dead);
    }
}

fn main() {
    let event_loop = EventLoop::new();

    let (window, window_size, dpi_factor) = make_window("Game of Life", &event_loop);
    let cell_size = dpi_factor * 2 as f32;
    let mut vgame = VisualGame::new(
        window_size.width as usize,
        window_size.height as usize,
        cell_size,
        &window,
    );
    let mut last_game_update = std::time::Instant::now();
    let mut last_render = Instant::now();
    let mut last_update_duration = Duration::new(0, 0); // Store the time required per update to enable more accurate frame and update intervals
    let mut modifier_state = ModifiersState::empty();

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
                    handle_keyboard_input(input, &modifier_state, &mut vgame, &window);
                }
                mouse_input @ WindowEvent::MouseInput { .. } => {}
                _ => {}
            },
            Event::MainEventsCleared => {
                let begin = Instant::now();
                let update_delay = begin - last_game_update;
                let render_delay = begin - last_render;
                if (update_delay + last_update_duration) >= vgame.update_time {
                    vgame.evolve();
                    last_game_update = Instant::now();
                    last_update_duration = last_game_update - begin;
                }
                if render_delay >= vgame.frame_time {
                    window.request_redraw();
                }
            }
            Event::RedrawRequested(_) => {
                vgame.update_pixel_buffer();
                if vgame.render().is_err() {
                    eprintln!("Error: Could not render to pixel buffer!");
                    *control_flow = ControlFlow::Exit;
                    return;
                } else {
                    last_render = Instant::now();
                }
            }
            _ => (),
        }
    });
}

fn handle_keyboard_input(
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
            game.step();
            window.request_redraw();
        }
        _ => {}
    }
}

fn make_window(title: &str, event_loop: &EventLoop<()>) -> (Window, dpi::PhysicalSize<u32>, f32) {
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
