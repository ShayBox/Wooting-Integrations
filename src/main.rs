use std::time::Duration;

use anyhow::Result;
#[allow(unused_imports)]
use wooting_integrations::integrations::prelude::*;
use wooting_integrations::{Command, Keyboard, Product};

fn main() -> Result<()> {
    loop {
        /* Keep searching for the keyboard until you find one */
        let Ok(keyboard) = Keyboard::find(Product::WootingTwoLe) else {
            eprintln!("Error: Couldn't find device, waiting...");
            std::thread::sleep(Duration::from_secs(1));
            continue;
        };

        /* Reset and initialize the keyboard */
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

        'restart: loop {
            if let Err(error) = keyboard.update(&mut |keyboard, rgb, pos| {
                #[cfg(feature = "animation")]
                animation.color(keyboard, rgb, pos);
                #[cfg(feature = "hyprland")]
                hyprland.color(keyboard, rgb, pos);
                #[cfg(feature = "mangohud")]
                mangohud.color(keyboard, rgb, pos);
                #[cfg(feature = "wooting")]
                wooting.color(keyboard, rgb, pos);
            }) {
                eprintln!("Error: {error}");
                break 'restart;
            }
        }
    }
}
