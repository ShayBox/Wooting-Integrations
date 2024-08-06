use anyhow::Result;
use wooting_integrations::{
    integrations::prelude::*, Command, KeyColor, Keyboard, Matrix, Product,
};

fn main() -> Result<()> {
    /* Try to find, reset, and initialize the device */
    let keyboard = Keyboard::find(Product::WootingTwoLe)?;
    keyboard.send_command(Command::WootDevResetAll, 0, 0, 0, 0)?;
    keyboard.send_command(Command::WootDevInit, 0, 0, 0, 0)?;

    let mut matrix = Matrix::default();
    #[cfg(feature = "rainbow")]
    let mut rainbow = Rainbow::default();
    #[cfg(feature = "hyprland")]
    let mut hyprland = Hyprland::default();

    loop {
        rainbow.next()?;
        hyprland.next()?;

        /* Wooting: State */
        let index = keyboard.send_command(Command::GetCurrentKeyboardProfileIndex, 0, 0, 0, 0)?[5];

        for (col, line) in matrix.iter_mut().enumerate() {
            for (row, pixel) in line.iter_mut().enumerate() {
                let mut rgba = [0u8; 4];

                rainbow.color(&mut rgba, (col, row));
                hyprland.color(&mut rgba, (col, row));

                /* Wooting: Iteration */
                if row == 0 && col == 17 + index as usize {
                    rgba = [u8::MAX; 4];
                }

                *pixel = KeyColor::from(rgba).0;
            }
        }

        keyboard.send_buffer(matrix)?;

        // std::thread::sleep(std::time::Duration::from_millis(15));
    }
}
