/*
 * Library example code:
 */
#[macro_use]
extern crate lazy_static;

extern crate sdl2;
extern crate sdl2_unifont;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2_unifont::renderer::SurfaceRenderer;
use std::boxed::Box;

// Rainbow text colours, from back to front
lazy_static! {
    static ref colours: [Box<Color>; 12] = [
        Box::new(Color::RGB(255, 0, 127)),
        Box::new(Color::RGB(255, 0, 255)),
        Box::new(Color::RGB(127, 0, 255)),
        Box::new(Color::RGB(0, 0, 255)),
        Box::new(Color::RGB(0, 127, 255)),
        Box::new(Color::RGB(0, 255, 255)),
        Box::new(Color::RGB(0, 255, 127)),
        Box::new(Color::RGB(0, 255, 0)),
        Box::new(Color::RGB(127, 255, 0)),
        Box::new(Color::RGB(255, 255, 0)),
        Box::new(Color::RGB(255, 127, 0)),
        Box::new(Color::RGB(255, 0, 0)),
    ];
}

/// Called from main to draw the demo text objects once at program start
fn draw_demo<'a>(iter_num: usize) -> sdl2::surface::Surface<'a> {
    // Where we'll blit all of our text surfaces onto
    let mut screen = sdl2::surface::Surface::new(
        800,
        600,
        sdl2::pixels::PixelFormatEnum::RGBA8888,
    ).unwrap();

    // Used to create surfaces containing rendered text
    let mut renderer =
        SurfaceRenderer::new(Color::RGB(0, 0, 0), Color::RGB(255, 255, 255));

    // The renderer draw method returns a surface, which we can use like any
    // other SDL surface
    renderer
        .draw("We can draw very simple text.")
        .unwrap()
        .blit(None, &mut screen, Rect::new(2, 2, 0, 0))
        .unwrap();

    // Higher-range Unicode characters also work.
    renderer
        .draw("全角文字も対応しています。")
        .unwrap()
        .blit(None, &mut screen, Rect::new(2, 20, 0, 0))
        .unwrap();

    // Text can be scaled by integer multiples
    renderer.scale = 3;
    renderer
        .draw("BIG text")
        .unwrap()
        .blit(None, &mut screen, Rect::new(100, 100, 0, 0))
        .unwrap();

    let nc = colours.len();
    // Make text background transparent for overlapping
    renderer.bg_color = Color::RGBA(255, 255, 255, 0);

    // Cycle through colours
    for (i, colour) in colours[iter_num % nc..nc]
        .into_iter()
        .chain(colours[0..iter_num % nc].into_iter())
        .enumerate()
    {
        // Text colour can be changed per-draw operation
        renderer.fg_color = **colour;
        renderer
            .draw("Rainbow text")
            .unwrap()
            .blit(
                None,
                &mut screen,
                Rect::new(
                    2 + nc as i32 - i as i32,
                    38 + nc as i32 - i as i32,
                    0,
                    0,
                ),
            ).unwrap();
    }

    screen
}

/*
 * SDL boilerplate (not really relevant for the library demo):
 */
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

fn main() {
    // SDL initialisers
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // Create window
    let window = video_subsystem
        .window("sdl2-unifont demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    // Create and clear window canvas
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    // Window stays black whilst font decompression / parsing occurs
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    canvas.set_draw_color(Color::RGB(255, 255, 255));

    // Event / render loop
    let mut iter_num: usize = 0;
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.clear();

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

        // Draw demo
        let demo_tex = texture_creator
            .create_texture_from_surface(draw_demo(iter_num))
            .unwrap();
        canvas.copy(&demo_tex, None, None).unwrap();

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 24));

        iter_num += 1;
    }
}
