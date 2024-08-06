use anyhow::Result;
#[allow(unused_imports)]
use wooting_integrations::integrations::prelude::*;
use wooting_integrations::{Command, KeyColor, Keyboard, Product};

fn main() -> Result<()> {
    let mut keyboard = Keyboard::find(Product::WootingTwoLe)?;
    keyboard.send_command(Command::WootDevResetAll, 0, 0, 0, 0)?;
    keyboard.send_command(Command::WootDevInit, 0, 0, 0, 0)?;

    #[cfg(feature = "rainbow")]
    let mut rainbow = Rainbow::default();
    #[cfg(feature = "hyprland")]
    let mut hyprland = Hyprland::default();
    #[cfg(feature = "wooting")]
    let mut wooting = Wooting::default();

    loop {
        #[cfg(feature = "rainbow")]
        rainbow.next(&keyboard)?;
        #[cfg(feature = "hyprland")]
        hyprland.next(&keyboard)?;
        #[cfg(feature = "wooting")]
        wooting.next(&keyboard)?;

        for (col, scanline) in keyboard.matrix.iter_mut().enumerate() {
            for (row, pixel) in scanline.iter_mut().enumerate() {
                #[allow(clippy::useless_let_if_seq)]
                let mut rgba = [0u8; 4];

                #[cfg(feature = "rainbow")]
                rainbow.color(&mut rgba, (col, row));
                #[cfg(feature = "hyprland")]
                hyprland.color(&mut rgba, (col, row));
                #[cfg(feature = "wooting")]
                wooting.color(&mut rgba, (col, row));

                *pixel = KeyColor::from(rgba).0;
            }
        }

        keyboard.send_rgb_matrix()?;
    }
}
