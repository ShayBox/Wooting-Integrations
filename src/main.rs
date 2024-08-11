use anyhow::Result;
#[allow(unused_imports)]
use wooting_integrations::integrations::prelude::*;
use wooting_integrations::{Command, Keyboard, Product};

fn main() -> Result<()> {
    loop {
        /* Keep searching for the keyboard until you find one */
        let Ok(keyboard) = Keyboard::find(Product::WootingTwoLe) else {
            eprintln!("Error: Couldn't find device");
            continue;
        };

        /* Reset and initialize the keyboard */
        keyboard.send_command(Command::WootDevResetAll)?;
        keyboard.send_command(Command::WootDevInit)?;

        /* The keyboard may lose connection while running */
        if let Err(error) = start_integrations_loop(&keyboard) {
            eprintln!("Error: {error}");
        }
    }
}

fn start_integrations_loop(keyboard: &Keyboard) -> Result<()> {
    let mut integrations: Vec<Box<dyn Integration>> = vec![
        #[cfg(feature = "animation")]
        Box::new(Animation::default()),
        #[cfg(feature = "hyprland")]
        Box::new(Hyprland::default()),
        #[cfg(feature = "mangohud")]
        Box::new(Mangohud::default()),
        #[cfg(feature = "wooting")]
        Box::new(Wooting::default()),
    ];

    loop {
        keyboard.update(&mut |keyboard, rgb, pos| {
            for integration in &mut integrations {
                integration.color(keyboard, rgb, pos);
            }
        })?;
    }
}
