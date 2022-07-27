use crate::globals;
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

pub fn make_window<T>(title: &str, event_loop: &EventLoop<T>) -> (Window, PhysicalSize<u32>, f32) {
    let window = WindowBuilder::new()
        .with_position(PhysicalPosition::new(0, 0))
        .with_visible(false)
        .with_title(title)
        .build(event_loop)
        .expect("Could not construct window!");
    let dpi_factor = window.scale_factor();
    let window_size = PhysicalSize::new(globals::DEFAULT_SIZE.0, globals::DEFAULT_SIZE.1);
    let monitor_size = if let Some(monitor) = window.current_monitor() {
        monitor.size()
    } else {
        window_size
    };
    // Scale window to 2/3 of monitor size
    let scale = (monitor_size.width as f64 / window_size.width as f64 * 2.0 / 3.0)
        .round()
        .max(1.0);
    let inner_size = PhysicalSize::new(
        window_size.width as f64 * scale,
        window_size.height as f64 * scale,
    );
    window.set_inner_size(inner_size);
    window.set_min_inner_size(Some(PhysicalSize::new(
        inner_size.width.floor(),
        inner_size.height.floor(),
    )));
    window.set_visible(true);
    (
        window,
        PhysicalSize::new(inner_size.width as u32, inner_size.height as u32),
        dpi_factor as f32,
    )
}
