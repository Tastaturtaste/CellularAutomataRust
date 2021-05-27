use std::convert::TryInto;

use crate::cell::Cell;
use crate::game::traits;
use crate::globals::*;
use pixels::Pixels;
use winit::window::Window;
pub struct Visuals {
    pixel_buffer: Pixels,
    window: Window,
    decay_multiplier: f32, // 1.0 -> instant decay, 0.0 -> never decay
}

impl Visuals {
    pub fn new(width: usize, height: usize, window: Window) -> Visuals {
        let win_size = window.inner_size();
        let surface = pixels::SurfaceTexture::new(win_size.width, win_size.height, &window);
        let pixel_buffer: Pixels = pixels::Pixels::new(
            width.try_into().expect(OVERFLOW_MSG),
            height.try_into().expect(OVERFLOW_MSG),
            surface,
        )
        .expect("Cannot create pixel texture!");
        let decay_multiplier = 1.0;

        Visuals {
            pixel_buffer,
            window,
            decay_multiplier,
        }
    }
    pub fn update_pixel_buffer<T: traits::CellGame>(
        &mut self,
        game: &T,
        overwrite_decaying: fn(&T::Cell) -> bool,
    ) {
        for (pixel, c) in self
            .pixel_buffer
            .get_frame()
            .chunks_exact_mut(4)
            .zip(game.get_board().into_iter())
        {
            let rgba = c.to_rgba();
            if overwrite_decaying(c) {
                pixel.copy_from_slice(&rgba.get_raw());
            } else {
                let decay_multiplier = self.decay_multiplier;
                pixel
                    .iter_mut()
                    .zip(rgba.get_raw().iter())
                    .for_each(|(byte, new_rgba)| {
                        *byte = (*byte as f32 * (1.0 - decay_multiplier)
                            + *new_rgba as f32 * decay_multiplier)
                            as u8
                    })
            }
        }
    }
    pub fn render(&mut self) -> Result<(), pixels::Error> {
        self.pixel_buffer.render()
    }
    pub fn get_decay_multiplier(&self) -> f32 {
        self.decay_multiplier
    }
    pub fn set_decay_multiplier(&mut self, decay_multiplier: f32) -> Result<(), &str> {
        if decay_multiplier > 1.0 || decay_multiplier < 0.0 {
            return Err("Out of bounds!");
        }
        Ok(self.decay_multiplier = decay_multiplier)
    }
    pub fn get_window(&self) -> &Window {
        &self.window
    }
    pub fn get_buffer(&self) -> &Pixels {
        &self.pixel_buffer
    }
    pub fn resize_surface(&mut self, width: u32, height: u32) {
        self.pixel_buffer.resize_surface(width, height)
    }
    pub fn resize_buffer(&mut self, width: u32, height: u32) {
        self.pixel_buffer.resize_buffer(width, height)
    }
}
