use std::{collections::HashMap, time::Instant};

use anyhow::Result;
use hyprland::{data::Workspace, shared::HyprDataActive};
use wooting_integrations::{
    Command, KeyColor, Keyboard, Matrix, Product, WOOTING_RGB_COLS, WOOTING_RGB_ROWS,
};

fn main() -> Result<()> {
    /* Try to find, reset, and initialize the device */
    let keyboard = Keyboard::find(Product::WootingTwoLe)?;
    keyboard.send_command(Command::WootDevResetAll, 0, 0, 0, 0)?;
    keyboard.send_command(Command::WootDevInit, 0, 0, 0, 0)?;

    // TODO: Split the Rainbow Animation and Hyprlanwwd integration into separate opt-out features

    /* Rainbow Animation: State */
    let instant = Instant::now();
    let gradient = colorgrad::sinebow();
    let mut matrix = Matrix::default();

    /* Hyprland: State */
    let mut workspaces = HashMap::new();

    loop {
        /* Rainbow Animation: Loop */
        let elapsed = instant.elapsed().as_secs_f64();
        let progress = (elapsed / -3.0) % 1.0;

        /* Hyprland: Loop */
        let active_workspace = Workspace::get_active()?;
        workspaces.insert(active_workspace.monitor_id, active_workspace.id);

        /* Wooting: State */
        let index = keyboard.send_command(Command::GetCurrentKeyboardProfileIndex, 0, 0, 0, 0)?[5];

        /* Loop over rows */
        for (row, matrix) in matrix.iter_mut().enumerate() {
            #[allow(clippy::cast_precision_loss)] // Cannot exceed f64 max
            let y = row as f64 / (WOOTING_RGB_ROWS - 1) as f64 / 5.0;

            /* Loop over columns */
            for (col, pixel) in matrix.iter_mut().enumerate() {
                #[allow(clippy::cast_precision_loss)] // Cannot exceed f64 max
                let x = col as f64 / (WOOTING_RGB_COLS - 1) as f64 / 2.0;

                /* Rainbow Animation: Iteration */
                let progress = (x + y + progress) % 1.0;
                let color = gradient.at(progress);
                let mut rgba = color.to_rgba8();

                /* Hyprland: Iteration */
                for workspace_id in workspaces.values() {
                    let key = match workspace_id {
                        1 => (4, 17),
                        2 => (4, 18),
                        3 => (4, 19),
                        4 => (3, 17),
                        5 => (3, 18),
                        6 => (3, 19),
                        7 => (2, 17),
                        8 => (2, 18),
                        9 => (2, 19),
                        _ => (usize::MAX, usize::MAX),
                    };

                    if key == (row, col) {
                        rgba = [u8::MAX; 4];
                    }
                }

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
