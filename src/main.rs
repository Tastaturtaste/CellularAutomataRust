use winit::{
    event::{Event, WindowEvent},
    event_loop::{EventLoop, ControlFlow},
    window::WindowBuilder,
};



fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Game of Life")
        .build(&event_loop)
        .expect("Window creation failed!");

    event_loop.run(
        move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested, .. 
                } => {
                    println!("The close button was pressed; stopping");
                    *control_flow = ControlFlow::Exit
                },
                Event::MainEventsCleared => {
                    window.request_redraw();
                },
                _ => ()
            }
        }
    );
}