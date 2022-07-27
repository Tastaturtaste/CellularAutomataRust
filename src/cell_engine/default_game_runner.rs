use crate::{dprintln, game::traits::CellGame as CellGameTrait, visuals::Visuals};
// use log::trace;
use std::time::{Duration, Instant};
use winit::{
    dpi::PhysicalPosition,
    event::{
        ElementState, Event, KeyboardInput, ModifiersState, MouseButton, VirtualKeyCode,
        WindowEvent,
    },
    event_loop::{ControlFlow, EventLoop, EventLoopProxy},
    window::{Fullscreen, WindowBuilder},
};
mod traits {
    pub trait UserEvent: PartialEq + Clone + Copy {}
}
pub struct MouseState {
    pub position: PhysicalPosition<f64>,
    pub left: ElementState,
    pub right: ElementState,
    pub middle: ElementState,
}
impl Default for MouseState {
    fn default() -> Self {
        Self {
            position: PhysicalPosition::default(),
            left: ElementState::Released,
            middle: ElementState::Released,
            right: ElementState::Released,
        }
    }
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
pub struct MouseInput {
    pub state: ElementState,
    pub button: MouseButton,
}
pub struct GameContext {
    last_cell_stepped: Option<(usize, usize)>,
    update_time: Duration,
    paused: bool,
    stop: bool,
}

pub struct GameRunner<CG: 'static + CellGameTrait> {
    overwrite_decaying: fn(&CG::Cell) -> bool,
    event_loop: EventLoop<UserEvent>,
}

impl<CG: CellGameTrait> GameRunner<CG> {
    pub fn new(decay_decider: fn(&CG::Cell) -> bool) -> Self {
        let event_loop = EventLoop::<UserEvent>::with_user_event();
        Self {
            overwrite_decaying: decay_decider,
            event_loop,
        }
    }

    pub fn run(self, mut game: CG, title: &str) {
        let game_dim = game.dim();
        let window = WindowBuilder::new()
            .with_position(PhysicalPosition::new(0, 0))
            .with_visible(false)
            .with_fullscreen(Some(Fullscreen::Borderless(None)))
            .with_title(title)
            .build(&self.event_loop)
            .expect("Could not construct fullscreen window!");
        //let dpi_scaling = window.scale_factor();
        let mut visuals = Visuals::new(game_dim.0, game_dim.1, window);
        let decay_decider = self.overwrite_decaying;
        let mut game_context = GameContext {
            last_cell_stepped: None,
            update_time: Duration::from_secs_f32(1. / 4.),
            paused: false,
            stop: false,
        };
        let mut mouse_state = MouseState::default();
        let mut modifier_state = ModifiersState::default();
        let event_loop_proxy = self.event_loop.create_proxy();
        let mut last_game_update = Instant::now();
        visuals.get_window().set_visible(true);
        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        println!("The close button was pressed; stopping");
                        *control_flow = ControlFlow::Exit;
                    }
                    WindowEvent::Resized(size) => {
                        visuals.resize_surface(size.width, size.height);
                        visuals.get_window().request_redraw();
                    }
                    WindowEvent::ModifiersChanged(state) => {
                        modifier_state = state;
                    }
                    WindowEvent::KeyboardInput { input, .. } => {
                        on_keyboard_input(
                            input,
                            &modifier_state,
                            &mut visuals,
                            &mut game,
                            &mut game_context,
                            &event_loop_proxy,
                        );
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        mouse_state.update_position(position);
                        on_mouse_state_updated(
                            &mouse_state,
                            &modifier_state,
                            &mut visuals,
                            &mut game_context,
                            &event_loop_proxy,
                        )
                    }
                    WindowEvent::MouseInput { button, state, .. } => {
                        mouse_state.update_button(MouseInput { button, state });
                        on_mouse_state_updated(
                            &mouse_state,
                            &modifier_state,
                            &mut visuals,
                            &mut game_context,
                            &event_loop_proxy,
                        )
                    }
                    _ => {}
                },
                Event::UserEvent(user_event) => on_user_event(
                    user_event,
                    &mut game,
                    &mut game_context,
                    &mut visuals,
                    &event_loop_proxy,
                ),
                Event::MainEventsCleared => {
                    if game_context.stop {
                        *control_flow = ControlFlow::Exit;
                    }
                    if !game_context.paused {
                        let begin = Instant::now();
                        let update_delay = begin - last_game_update;
                        if update_delay >= game_context.update_time {
                            game.step();
                            last_game_update = Instant::now();
                            visuals.get_window().request_redraw();
                        }
                    }
                }
                Event::RedrawRequested(_) => {
                    visuals.update_pixel_buffer(&game, decay_decider);
                    if visuals.render().is_err() {
                        eprintln!("Error: Could not render to pixel buffer!");
                        *control_flow = ControlFlow::Exit;
                    }
                }
                _ => (),
            }
        });
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum UserEvent {
    StepCell { x: usize, y: usize },
}
impl traits::UserEvent for UserEvent {}

fn on_mouse_state_updated(
    mouse_state: &MouseState,
    _modifier_state: &ModifiersState,
    visuals: &mut Visuals,
    game_context: &mut GameContext,
    event_loop_proxy: &EventLoopProxy<UserEvent>,
) {
    let (x, y) = visuals
        .get_buffer()
        .window_pos_to_pixel(mouse_state.position.into())
        .unwrap_or_else(|pos| visuals.get_buffer().clamp_pixel_pos(pos));
    if let ElementState::Pressed = mouse_state.left {
        dprintln!("Clicked at {}, {}", x, y);
        //trace!("Clicked at {}, {}", x, y);
        event_loop_proxy
            .send_event(UserEvent::StepCell { x, y })
            .expect("Sending event to a proxy event loop failed!");
    } else {
        game_context.last_cell_stepped = None;
    }
}

fn on_keyboard_input<T: CellGameTrait>(
    input: KeyboardInput,
    modifier_state: &ModifiersState,
    visuals: &mut Visuals,
    game: &mut T,
    game_context: &mut GameContext,
    _event_loop_proxy: &EventLoopProxy<UserEvent>,
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
        Some(VirtualKeyCode::P) => game_context.paused = !game_context.paused,
        Some(VirtualKeyCode::PageUp) => {
            match *modifier_state {
                ModifiersState::SHIFT => {
                    // Since a higher value implies a higher decay rate we add to make decay faster
                    let mut decay_multi = visuals.get_decay_multiplier();
                    decay_multi += 0.1;
                    decay_multi = decay_multi.clamp(0.0, 1.0);
                    visuals
                        .set_decay_multiplier(decay_multi)
                        .expect("Decay multiplier should only be between 0 and 1!");
                    println!("Increased decay rate to {}", decay_multi);
                }
                _ => {
                    game_context.update_time = game_context.update_time.mul_f32(0.9);
                    println!("Decreased update_time");
                }
            }
        }
        Some(VirtualKeyCode::PageDown) => {
            match *modifier_state {
                ModifiersState::SHIFT => {
                    // Since a bigger value implies a lower decay rate we subtract to make decay slower
                    let mut decay_multi = visuals.get_decay_multiplier();
                    decay_multi -= 0.1;
                    decay_multi = decay_multi.clamp(0.0, 1.0);
                    visuals
                        .set_decay_multiplier(decay_multi)
                        .expect("Decay multiplier should only be between 0 and 1!");
                    println!("Decreased decay rate to {}", decay_multi);
                }
                _ => {
                    game_context.update_time = game_context.update_time.mul_f32(1.1);
                    println!("Increased update_time")
                }
            }
        }
        Some(VirtualKeyCode::Space) => {
            game.step();
            visuals.get_window().request_redraw();
        }
        Some(VirtualKeyCode::Escape) => game_context.stop = true,
        _ => {}
    }
}

fn on_user_event<T: CellGameTrait>(
    event: UserEvent,
    game: &mut T,
    game_context: &mut GameContext,
    visuals: &mut Visuals,
    _event_loop_proxy: &EventLoopProxy<UserEvent>,
) {
    match event {
        UserEvent::StepCell { x, y } => {
            if let Some((last_x, last_y)) = game_context.last_cell_stepped {
                if last_x == x && last_y == y {
                    return;
                }
            }
            game.next_cell(x, y);
            game_context.last_cell_stepped = Some((x, y));
            visuals.get_window().request_redraw();
        }
    }
}
