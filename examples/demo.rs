/*
 * Library example code:
 */
extern crate sdl2;
extern crate sdl2_unifont;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2_unifont::renderer::SurfaceRenderer;

/// Called from main to draw the demo text objects once at program start
fn draw_demo<'a>() -> sdl2::surface::Surface<'a> {
    // Where we'll blit all of our text surfaces onto
    let mut screen = sdl2::surface::Surface::new(
        800,
        600,
        sdl2::pixels::PixelFormatEnum::RGBA8888,
    ).unwrap();

    // Used to create surfaces containing rendered text
    let mut renderer =
        SurfaceRenderer::new(Color::RGB(0, 0, 0), Color::RGB(255, 255, 255));
    renderer.scale = 1;

    renderer
        .draw("We can draw very simple text.")
        .unwrap()
        .blit(None, &mut screen, Rect::new(2, 2, 0, 0))
        .unwrap();

    renderer
        .draw("全角文字も対応しています。")
        .unwrap()
        .blit(None, &mut screen, Rect::new(2, 20, 0, 0))
        .unwrap();

    screen
}

/*
 * SDL boilerplate (not really relevant for the library demo):
 */
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("sdl2-unifont demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    // Create and clear window canvas
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();

    // Create texture creator, convert demo surface to texture, copy to window
    let texture_creator = canvas.texture_creator();
    let demo_tex = texture_creator
        .create_texture_from_surface(draw_demo())
        .unwrap();
    canvas.copy(&demo_tex, None, None).unwrap();

    canvas.present();

    // Event loop - no rendering is performed here
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }
}
