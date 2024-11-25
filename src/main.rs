extern crate sdl2;

use sdl2::event::Event;
use sdl2::image::InitFlag;
use sdl2::keyboard::Keycode;
use sdl2::pixels;
use std::time::{Duration, Instant};

use sdl2::gfx::primitives::DrawRenderer;

const SCREEN_WIDTH: u32 = 1920;
const SCREEN_HEIGHT: u32 = 1080;
struct Source {
    x: u32,
    y: u32,
    amp: f32,
    k: f32,
    omega: f32,
    phi: f32,
}
impl Source {
    fn rayonnement(&self, x: u32, y: u32, t: f32) -> f32 {
        return self.amp
            * (self.phi - self.omega * t
                + self.k
                    * ((x as i32 - self.x as i32).pow(2) as f32
                        + (y as i32 - self.y as i32).pow(2) as f32)
                        .sqrt() as f32)
                .sin();
    }
}
fn interference(liste_sources: &[Source], x: u32, y: u32, t: f32) -> sdl2::pixels::Color {
    return intensite_couleur(
        liste_sources
            .iter()
            .fold(0., |acc, s| acc + s.rayonnement(x, y, t)),
    );
}
fn intensite_couleur(i: f32) -> sdl2::pixels::Color {
    return pixels::Color::RGB(0, 0, (127. + 127. * i) as u8);
}
fn run(liste_sources: &[Source]) -> Result<(), String> {
    let now = Instant::now();
    let sdl_context = sdl2::init()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    let video_subsys = sdl_context.video()?;
    let window = video_subsys
        .window(
            "rust-sdl2_gfx: draw line & FPSManager",
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
        )
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())?;

    canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    // let mut lastx = 0;
    // let mut lasty = 0;

    let mut events = sdl_context.event_pump()?;
    'main: loop {
        canvas.clear();
        let rendus = Instant::now();
        for x in 0..SCREEN_WIDTH {
            for y in 0..SCREEN_HEIGHT {
                canvas.pixel(
                    x as i16,
                    y as i16,
                    interference(liste_sources, x, y, now.elapsed().as_secs_f32()),
                )?;
            }
        }
        println!("{} secondes pour le rendus", rendus.elapsed().as_secs_f32());
        canvas.present();
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,

                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if keycode == Keycode::Escape {
                        break 'main;
                    }
                }
                //     } else if keycode == Keycode::Space {
                //         println!("space down");
                //     }
                // }

                // Event::MouseButtonDown { x, y, .. } => {
                //     let color = pixels::Color::RGB(x as u8, y as u8, 255);
                //     let _ = canvas.line(lastx, lasty, x as i16, y as i16, color);
                //     lastx = x as i16;
                //     lasty = y as i16;
                //     println!("mouse btn down at ({},{})", x, y);
                //     canvas.present();
                // }
                _ => {}
            }
        }
    }

    Ok(())
}
fn main() -> Result<(), String> {
    let k = 0.3;
    let omega = 5.;
    let n = 10;
    let mut liste_sources = Vec::with_capacity(n);
    for i in 0..n {
        liste_sources.push(Source {
            x: SCREEN_WIDTH / 2 + (i as f32 * 3.14 / k) as u32,
            y: SCREEN_HEIGHT / 2,
            amp: 1. / n as f32,
            omega: omega,
            k: k,
            phi: 0.,
        });
    }
    run(&liste_sources);

    Ok(())
}
