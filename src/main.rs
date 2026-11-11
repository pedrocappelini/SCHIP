mod emulator;

use emulator::Schip;
use macroquad::prelude::*;

#[macroquad::main(window_config)]
async fn main() {
    let mut schip = Schip::new();
    schip.load_rom("roms/rockto.ch8");
    emulator::run(schip).await;
}

fn window_config() -> Conf {
    Conf {
        window_title: "Chip-8".to_string(),
        window_width: 1280,
        window_height: 640,
        ..Default::default()
    }
}
