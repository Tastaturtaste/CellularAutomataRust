mod game_board;
mod game_rules;
mod game;
mod cell;
mod globals;

use core::f32;

use game::*;
use game_board::*;
use cell::*;

use winit::{dpi::{self, PhysicalPosition}, event, event_loop, window};
use winit::event::Event as WinEvent;
use pixels;

pub struct RGBA([u8;4]);

impl RGBA{
    pub fn black() -> RGBA {
        RGBA{0:[0x00,0x00,0x00,0x00]}
    }
    pub fn white() -> RGBA {
        RGBA{0:[0xFF,0xFF,0xFF,0xFF]}
    }
    pub fn get_raw(&self) -> [u8;4] {
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
}

impl VisualGame {
    pub fn new(width: u16, height: u16, dpi_factor: f32, window: &window::Window) -> VisualGame {
        let surface = pixels::SurfaceTexture::new(width as u32, height as u32, window);
        let mut pixel_buffer = pixels::Pixels::new(width.into() ,height.into() ,surface).expect("Cannot create pixel texture!");
        let width = (width as f32 / dpi_factor).round() as u16;
        let height = (height as f32 / dpi_factor).round() as u16;
        pixel_buffer.resize_buffer(width as u32, height as u32);
        VisualGame{
            game: ConwaysGame::new_rand(width as u16, height as u16),
            pixel_buffer,
        }
    }
    pub fn evolve(&mut self) -> &GameBoard<CellConway> {
        self.game.evolve()
    }
    pub fn update_pixel_buffer(&mut self) {
        for (pixel, rgba) in self.pixel_buffer
        .get_frame()
        .chunks_exact_mut(4)
        .zip(self.game.get_board().into_iter().map(|c| c.to_rgba()))
        {
            pixel.copy_from_slice(&rgba.get_raw());
        }
    }
    pub fn render(&mut self) -> Result<(), pixels::Error> {
        self.pixel_buffer.render()
    }
}

pub struct WindowWrapper{
    window: window::Window,
    size: dpi::Size,
    dpi_factor: f32,
}
fn make_window(title: &str, event_loop: &event_loop::EventLoop<()>) -> (window::Window, dpi::PhysicalSize<u32>, f32)
{
    let window = window::WindowBuilder::new()
    .with_position(PhysicalPosition::new(0,0))
    .with_visible(false)
    .with_title(title)
    .build(event_loop).expect("Could not construct window!");
    let dpi_factor = window.scale_factor();
    let window_size = dpi::PhysicalSize::new(globals::DEFAULT_SIZE.0, globals::DEFAULT_SIZE.1);
    let monitor_size = if let Some(monitor) = window.current_monitor(){
        monitor.size()
    }
    else {
        window_size
    };
    // Scale window to 2/3 of monitor size
    let scale = (monitor_size.width as f64 / window_size.width as f64 * 2.0 / 3.0).round().max(1.0);
    let inner_size = dpi::PhysicalSize::new(window_size.width as f64 * scale, window_size.height as f64 * scale);
    window.set_inner_size(inner_size);
    window.set_min_inner_size(Some(dpi::PhysicalSize::new(inner_size.width.floor(), inner_size.height.floor())));
    window.set_visible(true);
    (
        window, 
        dpi::PhysicalSize::new(inner_size.width as u32, inner_size.height as u32), 
        dpi_factor as f32
    )
}

fn main() {
    let event_loop = event_loop::EventLoop::new();
    let (window, window_size, dpi_factor) = make_window("Game of Life", &event_loop);
    let mut vgame = VisualGame::new(window_size.width as u16, window_size.height as u16, dpi_factor, &window);

    event_loop.run(
        move |event, _, control_flow| {
            *control_flow = event_loop::ControlFlow::Poll;

            match event {
                WinEvent::WindowEvent {
                    event: event::WindowEvent::CloseRequested, .. 
                } => {
                    println!("The close button was pressed; stopping");
                    *control_flow = event_loop::ControlFlow::Exit;
                },
                WinEvent::MainEventsCleared => {
                    window.request_redraw();
                },
                WinEvent::RedrawRequested(_) => {
                    vgame.evolve();
                    vgame.update_pixel_buffer();
                    if vgame.render().is_err(){
                        *control_flow = event_loop::ControlFlow::Exit;
                        return;
                    }
                },
                _ => (),
            }
        }
    );
}