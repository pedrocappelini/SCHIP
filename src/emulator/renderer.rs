use macroquad::prelude::*;

use super::{Schip, input};

pub async fn run(mut chip8: Schip) {
    loop {
        if chip8.exit {
            break;
        }

        input::update(&mut chip8);

        for _ in 0..10 {
            chip8.cycles();
        }

        if chip8.d_timer > 0 {
            chip8.d_timer -= 1;
        }

        if chip8.s_timer > 0 {
            chip8.s_timer -= 1;
        }

        render_frame(&chip8);
        next_frame().await;
    }
}

fn render_frame(chip8: &Schip) {
    clear_background(BLACK);

    let (width, height) = if chip8.high_res {
        (128usize, 64usize)
    } else {
        (64usize, 32usize)
    };

    let square_width = screen_width() / width as f32;
    let square_height = screen_height() / height as f32;

    for y in 0..height {
        for x in 0..width {
            if chip8.pixel(x, y) {
                draw_rectangle(
                    x as f32 * square_width,
                    y as f32 * square_height,
                    square_width,
                    square_height,
                    WHITE,
                );
            }
        }
    }
}
