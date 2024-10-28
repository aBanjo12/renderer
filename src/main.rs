use std::time::Duration;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

fn render(canvas: &mut WindowCanvas, color: Color) {
    canvas.set_draw_color(color);
    canvas.clear();
    canvas.present();
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().expect("Failed to init SDL2");
    let video_subsystem = sdl_context.video().expect("Failed to init SDL2");

    let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600).position_centered().opengl().build().expect("Failed to open window");
    let mut canvas = window.into_canvas().build().expect("Failed to convert window to canvas");

    let mut event_pump = sdl_context.event_pump().expect("Failed to convert window to event pump");
    let mut i = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }

        i = (i + 1) % 255;

        render(&mut canvas, Color::RGB(i, 64, 255 - i));

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
