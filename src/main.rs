use std::time::Duration;

#[allow(unused_imports)]
use wooting_integrations::integrations::prelude::*;
use wooting_integrations::wooting::{Command, Keyboard, Product};

fn main() -> anyhow::Result<()> {
    loop {
        /* Check if Wootility is running */
        if wooting_integrations::is_wootility_running() {
            eprintln!("Error: Wootility process is running, waiting...");
            std::thread::sleep(Duration::from_secs(1));
            continue;
        }

        /* Keep searching for the keyboard until you find one */
        let Ok(keyboard) = Keyboard::find(Product::WootingTwoLe) else {
            eprintln!("Error: Couldn't find device, waiting...");
            std::thread::sleep(Duration::from_secs(1));
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

fn start_integrations_loop(keyboard: &Keyboard) -> anyhow::Result<()> {
    #[cfg(feature = "latency")]
    let mut latency = Latency::default();
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
        #[cfg(feature = "latency")]
        let instant = std::time::Instant::now();

        keyboard.update(&mut |keyboard, rgb, pos| {
            for integration in &mut integrations {
                integration.color(keyboard, rgb, pos);
            }
        })?;

        #[cfg(feature = "latency")]
        {
            use std::io::Write;

            let elapsed = instant.elapsed();
            let average = latency.average();
            latency.add(elapsed);

            println!("\rLatency: {elapsed:.1?}{}", " ".repeat(8));
            print!("\rAverage Latency: {average:.1?} ");
            std::io::stdout().flush()?;
        }
    }
}
