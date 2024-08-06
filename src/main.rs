use anyhow::Result;
#[allow(unused_imports)]
use wooting_integrations::integrations::prelude::*;
use wooting_integrations::{Command, Keyboard, Product};

fn main() -> Result<()> {
    let keyboard = Keyboard::find(Product::WootingTwoLe)?;
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

        keyboard.update(&mut |keyboard, rgb, pos| {
            #[cfg(feature = "animation")]
            animation.color(keyboard, rgb, pos);
            #[cfg(feature = "hyprland")]
            hyprland.color(keyboard, rgb, pos);
            #[cfg(feature = "mangohud")]
            mangohud.color(keyboard, rgb, pos);
            #[cfg(feature = "wooting")]
            wooting.color(keyboard, rgb, pos);
        })?;

        // let millis = start.elapsed().as_millis();
        // if millis > 16 {
        //     println!("{millis}");
        // }
    }
}
