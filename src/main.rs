use anyhow::Result;
#[allow(unused_imports)]
use wooting_integrations::integrations::prelude::*;
use wooting_integrations::{Command, KeyColor, Keyboard, Product};

fn main() -> Result<()> {
    let mut keyboard = Keyboard::find(Product::WootingTwoLe)?;
    keyboard.send_command(Command::WootDevResetAll, 0, 0, 0, 0)?;
    keyboard.send_command(Command::WootDevInit, 0, 0, 0, 0)?;

    #[cfg(feature = "animation")]
    let mut animation = Animation::default();
    #[cfg(feature = "hyprland")]
    let mut hyprland = Hyprland::default();
    #[cfg(feature = "mangohud")]
    let mut mangohud = Mangohud::default();
    #[cfg(feature = "wooting")]
    let mut wooting = Wooting::default();

    loop {
        // let start = std::time::Instant::now();

        let mut matrix = keyboard.matrix;
        for (col, scanline) in matrix.iter_mut().enumerate() {
            for (row, pixel) in scanline.iter_mut().enumerate() {
                #[allow(clippy::useless_let_if_seq)]
                let mut rgba = [0u8; 4];

                #[cfg(feature = "animation")]
                animation.color(&mut rgba, (col, row), &keyboard);
                #[cfg(feature = "hyprland")]
                hyprland.color(&mut rgba, (col, row), &keyboard);
                #[cfg(feature = "mangohud")]
                mangohud.color(&mut rgba, (col, row), &keyboard);
                #[cfg(feature = "wooting")]
                wooting.color(&mut rgba, (col, row), &keyboard);

                *pixel = KeyColor::from(rgba).0;
            }
        }

        keyboard.matrix = matrix;
        keyboard.send_rgb_matrix()?;

        // println!("{}", start.elapsed().as_millis());
    }
}
