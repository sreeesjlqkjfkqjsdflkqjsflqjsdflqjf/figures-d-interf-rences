extern crate sdl2;

use sdl2::event::Event;
use sdl2::image::InitFlag;
use sdl2::keyboard::Keycode;
use sdl2::pixels;
use std::{
    time::{Duration, Instant},
    usize,
};

use sdl2::gfx::primitives::DrawRenderer;

const SCREEN_WIDTH: u32 = 900;
const SCREEN_HEIGHT: u32 = 900;
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
        let dist = ((x as i32 - self.x as i32).pow(2) + (y as i32 - self.y as i32).pow(2));
        // return (self.amp) * (self.phi - self.omega * t + self.k).sin();
        return 0 as f32;
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
    let turbo_colormap_data = [
        pixels::Color::RGB(48, 18, 59),
        pixels::Color::RGB(49, 21, 66),
        pixels::Color::RGB(50, 24, 74),
        pixels::Color::RGB(52, 27, 81),
        pixels::Color::RGB(53, 30, 88),
        pixels::Color::RGB(54, 33, 95),
        pixels::Color::RGB(55, 35, 101),
        pixels::Color::RGB(56, 38, 108),
        pixels::Color::RGB(57, 41, 114),
        pixels::Color::RGB(58, 44, 121),
        pixels::Color::RGB(59, 47, 127),
        pixels::Color::RGB(60, 50, 133),
        pixels::Color::RGB(60, 53, 139),
        pixels::Color::RGB(61, 55, 145),
        pixels::Color::RGB(62, 58, 150),
        pixels::Color::RGB(63, 61, 156),
        pixels::Color::RGB(64, 64, 161),
        pixels::Color::RGB(64, 67, 166),
        pixels::Color::RGB(65, 69, 171),
        pixels::Color::RGB(65, 72, 176),
        pixels::Color::RGB(66, 75, 181),
        pixels::Color::RGB(67, 78, 186),
        pixels::Color::RGB(67, 80, 190),
        pixels::Color::RGB(67, 83, 194),
        pixels::Color::RGB(68, 86, 199),
        pixels::Color::RGB(68, 88, 203),
        pixels::Color::RGB(69, 91, 206),
        pixels::Color::RGB(69, 94, 210),
        pixels::Color::RGB(69, 96, 214),
        pixels::Color::RGB(69, 99, 217),
        pixels::Color::RGB(70, 102, 221),
        pixels::Color::RGB(70, 104, 224),
        pixels::Color::RGB(70, 107, 227),
        pixels::Color::RGB(70, 109, 230),
        pixels::Color::RGB(70, 112, 232),
        pixels::Color::RGB(70, 115, 235),
        pixels::Color::RGB(70, 117, 237),
        pixels::Color::RGB(70, 120, 240),
        pixels::Color::RGB(70, 122, 242),
        pixels::Color::RGB(70, 125, 244),
        pixels::Color::RGB(70, 127, 246),
        pixels::Color::RGB(70, 130, 248),
        pixels::Color::RGB(69, 132, 249),
        pixels::Color::RGB(69, 135, 251),
        pixels::Color::RGB(69, 137, 252),
        pixels::Color::RGB(68, 140, 253),
        pixels::Color::RGB(67, 142, 253),
        pixels::Color::RGB(66, 145, 254),
        pixels::Color::RGB(65, 147, 254),
        pixels::Color::RGB(64, 150, 254),
        pixels::Color::RGB(63, 152, 254),
        pixels::Color::RGB(62, 155, 254),
        pixels::Color::RGB(60, 157, 253),
        pixels::Color::RGB(59, 160, 252),
        pixels::Color::RGB(57, 162, 252),
        pixels::Color::RGB(56, 165, 251),
        pixels::Color::RGB(54, 168, 249),
        pixels::Color::RGB(52, 170, 248),
        pixels::Color::RGB(51, 172, 246),
        pixels::Color::RGB(49, 175, 245),
        pixels::Color::RGB(47, 177, 243),
        pixels::Color::RGB(45, 180, 241),
        pixels::Color::RGB(43, 182, 239),
        pixels::Color::RGB(42, 185, 237),
        pixels::Color::RGB(40, 187, 235),
        pixels::Color::RGB(38, 189, 233),
        pixels::Color::RGB(37, 192, 230),
        pixels::Color::RGB(35, 194, 228),
        pixels::Color::RGB(33, 196, 225),
        pixels::Color::RGB(32, 198, 223),
        pixels::Color::RGB(30, 201, 220),
        pixels::Color::RGB(29, 203, 218),
        pixels::Color::RGB(28, 205, 215),
        pixels::Color::RGB(27, 207, 212),
        pixels::Color::RGB(26, 209, 210),
        pixels::Color::RGB(25, 211, 207),
        pixels::Color::RGB(24, 213, 204),
        pixels::Color::RGB(24, 215, 202),
        pixels::Color::RGB(23, 217, 199),
        pixels::Color::RGB(23, 218, 196),
        pixels::Color::RGB(23, 220, 194),
        pixels::Color::RGB(23, 222, 191),
        pixels::Color::RGB(24, 224, 189),
        pixels::Color::RGB(24, 225, 186),
        pixels::Color::RGB(25, 227, 184),
        pixels::Color::RGB(26, 228, 182),
        pixels::Color::RGB(27, 229, 180),
        pixels::Color::RGB(29, 231, 177),
        pixels::Color::RGB(30, 232, 175),
        pixels::Color::RGB(32, 233, 172),
        pixels::Color::RGB(34, 235, 169),
        pixels::Color::RGB(36, 236, 166),
        pixels::Color::RGB(39, 237, 163),
        pixels::Color::RGB(41, 238, 160),
        pixels::Color::RGB(44, 239, 157),
        pixels::Color::RGB(47, 240, 154),
        pixels::Color::RGB(50, 241, 151),
        pixels::Color::RGB(53, 243, 148),
        pixels::Color::RGB(56, 244, 145),
        pixels::Color::RGB(59, 244, 141),
        pixels::Color::RGB(63, 245, 138),
        pixels::Color::RGB(66, 246, 135),
        pixels::Color::RGB(70, 247, 131),
        pixels::Color::RGB(74, 248, 128),
        pixels::Color::RGB(77, 249, 124),
        pixels::Color::RGB(81, 249, 121),
        pixels::Color::RGB(85, 250, 118),
        pixels::Color::RGB(89, 251, 114),
        pixels::Color::RGB(93, 251, 111),
        pixels::Color::RGB(97, 252, 108),
        pixels::Color::RGB(101, 252, 104),
        pixels::Color::RGB(105, 253, 101),
        pixels::Color::RGB(109, 253, 98),
        pixels::Color::RGB(113, 253, 95),
        pixels::Color::RGB(116, 254, 92),
        pixels::Color::RGB(120, 254, 89),
        pixels::Color::RGB(124, 254, 86),
        pixels::Color::RGB(128, 254, 83),
        pixels::Color::RGB(132, 254, 80),
        pixels::Color::RGB(135, 254, 77),
        pixels::Color::RGB(139, 254, 75),
        pixels::Color::RGB(142, 254, 72),
        pixels::Color::RGB(146, 254, 70),
        pixels::Color::RGB(149, 254, 68),
        pixels::Color::RGB(152, 254, 66),
        pixels::Color::RGB(155, 253, 64),
        pixels::Color::RGB(158, 253, 62),
        pixels::Color::RGB(161, 252, 61),
        pixels::Color::RGB(164, 252, 59),
        pixels::Color::RGB(166, 251, 58),
        pixels::Color::RGB(169, 251, 57),
        pixels::Color::RGB(172, 250, 55),
        pixels::Color::RGB(174, 249, 55),
        pixels::Color::RGB(177, 248, 54),
        pixels::Color::RGB(179, 248, 53),
        pixels::Color::RGB(182, 247, 53),
        pixels::Color::RGB(185, 245, 52),
        pixels::Color::RGB(187, 244, 52),
        pixels::Color::RGB(190, 243, 52),
        pixels::Color::RGB(192, 242, 51),
        pixels::Color::RGB(195, 241, 51),
        pixels::Color::RGB(197, 239, 51),
        pixels::Color::RGB(200, 238, 51),
        pixels::Color::RGB(202, 237, 51),
        pixels::Color::RGB(205, 235, 52),
        pixels::Color::RGB(207, 234, 52),
        pixels::Color::RGB(209, 232, 52),
        pixels::Color::RGB(212, 231, 53),
        pixels::Color::RGB(214, 229, 53),
        pixels::Color::RGB(216, 227, 53),
        pixels::Color::RGB(218, 226, 54),
        pixels::Color::RGB(221, 224, 54),
        pixels::Color::RGB(223, 222, 54),
        pixels::Color::RGB(225, 220, 55),
        pixels::Color::RGB(227, 218, 55),
        pixels::Color::RGB(229, 216, 56),
        pixels::Color::RGB(231, 215, 56),
        pixels::Color::RGB(232, 213, 56),
        pixels::Color::RGB(234, 211, 57),
        pixels::Color::RGB(236, 209, 57),
        pixels::Color::RGB(237, 207, 57),
        pixels::Color::RGB(239, 205, 57),
        pixels::Color::RGB(240, 203, 58),
        pixels::Color::RGB(242, 200, 58),
        pixels::Color::RGB(243, 198, 58),
        pixels::Color::RGB(244, 196, 58),
        pixels::Color::RGB(246, 194, 58),
        pixels::Color::RGB(247, 192, 57),
        pixels::Color::RGB(248, 190, 57),
        pixels::Color::RGB(249, 188, 57),
        pixels::Color::RGB(249, 186, 56),
        pixels::Color::RGB(250, 183, 55),
        pixels::Color::RGB(251, 181, 55),
        pixels::Color::RGB(251, 179, 54),
        pixels::Color::RGB(252, 176, 53),
        pixels::Color::RGB(252, 174, 52),
        pixels::Color::RGB(253, 171, 51),
        pixels::Color::RGB(253, 169, 50),
        pixels::Color::RGB(253, 166, 49),
        pixels::Color::RGB(253, 163, 48),
        pixels::Color::RGB(254, 161, 47),
        pixels::Color::RGB(254, 158, 46),
        pixels::Color::RGB(254, 155, 45),
        pixels::Color::RGB(254, 152, 44),
        pixels::Color::RGB(253, 149, 43),
        pixels::Color::RGB(253, 146, 41),
        pixels::Color::RGB(253, 143, 40),
        pixels::Color::RGB(253, 140, 39),
        pixels::Color::RGB(252, 137, 38),
        pixels::Color::RGB(252, 134, 36),
        pixels::Color::RGB(251, 131, 35),
        pixels::Color::RGB(251, 128, 34),
        pixels::Color::RGB(250, 125, 32),
        pixels::Color::RGB(250, 122, 31),
        pixels::Color::RGB(249, 119, 30),
        pixels::Color::RGB(248, 116, 28),
        pixels::Color::RGB(247, 113, 27),
        pixels::Color::RGB(247, 110, 26),
        pixels::Color::RGB(246, 107, 24),
        pixels::Color::RGB(245, 104, 23),
        pixels::Color::RGB(244, 101, 22),
        pixels::Color::RGB(243, 99, 21),
        pixels::Color::RGB(242, 96, 20),
        pixels::Color::RGB(241, 93, 19),
        pixels::Color::RGB(239, 90, 17),
        pixels::Color::RGB(238, 88, 16),
        pixels::Color::RGB(237, 85, 15),
        pixels::Color::RGB(236, 82, 14),
        pixels::Color::RGB(234, 80, 13),
        pixels::Color::RGB(233, 77, 13),
        pixels::Color::RGB(232, 75, 12),
        pixels::Color::RGB(230, 73, 11),
        pixels::Color::RGB(229, 70, 10),
        pixels::Color::RGB(227, 68, 10),
        pixels::Color::RGB(226, 66, 9),
        pixels::Color::RGB(224, 64, 8),
        pixels::Color::RGB(222, 62, 8),
        pixels::Color::RGB(221, 60, 7),
        pixels::Color::RGB(219, 58, 7),
        pixels::Color::RGB(217, 56, 6),
        pixels::Color::RGB(215, 54, 6),
        pixels::Color::RGB(214, 52, 5),
        pixels::Color::RGB(212, 50, 5),
        pixels::Color::RGB(210, 48, 5),
        pixels::Color::RGB(208, 47, 4),
        pixels::Color::RGB(206, 45, 4),
        pixels::Color::RGB(203, 43, 3),
        pixels::Color::RGB(201, 41, 3),
        pixels::Color::RGB(199, 40, 3),
        pixels::Color::RGB(197, 38, 2),
        pixels::Color::RGB(195, 36, 2),
        pixels::Color::RGB(192, 35, 2),
        pixels::Color::RGB(190, 33, 2),
        pixels::Color::RGB(187, 31, 1),
        pixels::Color::RGB(185, 30, 1),
        pixels::Color::RGB(182, 28, 1),
        pixels::Color::RGB(180, 27, 1),
        pixels::Color::RGB(177, 25, 1),
        pixels::Color::RGB(174, 24, 1),
        pixels::Color::RGB(172, 22, 1),
        pixels::Color::RGB(169, 21, 1),
        pixels::Color::RGB(166, 20, 1),
        pixels::Color::RGB(163, 18, 1),
        pixels::Color::RGB(160, 17, 1),
        pixels::Color::RGB(157, 16, 1),
        pixels::Color::RGB(154, 14, 1),
        pixels::Color::RGB(151, 13, 1),
        pixels::Color::RGB(148, 12, 1),
        pixels::Color::RGB(145, 11, 1),
        pixels::Color::RGB(142, 10, 1),
        pixels::Color::RGB(139, 9, 1),
        pixels::Color::RGB(135, 8, 1),
        pixels::Color::RGB(132, 7, 1),
        pixels::Color::RGB(129, 6, 2),
        pixels::Color::RGB(125, 5, 2),
        pixels::Color::RGB(122, 4, 2),
    ];
    return turbo_colormap_data[(127. + 127. * i) as usize];
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
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(None, SCREEN_WIDTH, SCREEN_HEIGHT)
        .unwrap();
    let mut pique_sels = [0; (4 * SCREEN_HEIGHT * SCREEN_WIDTH) as usize];
    'main: loop {
        let rendus = Instant::now();
        let t = now.elapsed().as_secs_f32();
        for x in 0..SCREEN_WIDTH {
            for y in 0..SCREEN_HEIGHT {
                let c = interference(liste_sources, x, y, t);
                pique_sels[((x + SCREEN_WIDTH * y) * 4) as usize] = c.b;
                pique_sels[((x + SCREEN_WIDTH * y) * 4 + 1) as usize] = c.g;
                pique_sels[((x + SCREEN_WIDTH * y) * 4 + 2) as usize] = c.r;
                pique_sels[((x + SCREEN_WIDTH * y) * 4 + 3) as usize] = c.a;
            }
        }
        println!("{} secondes pour le rendus", rendus.elapsed().as_secs_f32());
        texture
            .update(None, &pique_sels, (SCREEN_WIDTH * 4) as usize)
            .unwrap();
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();
        let reste = Instant::now();
        let mut events = sdl_context.event_pump()?;
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
        println!("{} secondes pour le reste", reste.elapsed().as_secs_f32());
    }

    Ok(())
}
fn main() -> Result<(), String> {
    let k = 0.3;
    let omega = 5.;
    let n = 2;
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
