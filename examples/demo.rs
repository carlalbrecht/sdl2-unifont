extern crate sdl2;
extern crate sdl2_unifont;

use sdl2::pixels::Color;

use sdl2_unifont::renderer::SurfaceRenderer;

fn main() {
    let renderer =
        SurfaceRenderer::new(Color::RGB(255, 255, 255), Color::RGB(0, 0, 0));

    println!("{}", renderer.measure_width("test").unwrap());
}
