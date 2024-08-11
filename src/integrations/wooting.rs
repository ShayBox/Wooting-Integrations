use std::time::Duration;

use memoize::memoize;

use crate::{Command, Keyboard, Rgb};

use super::Integration;

#[derive(Default)]
pub struct Wooting(());

impl Integration for Wooting {
    fn color(&mut self, keyboard: &Keyboard, rgb: &mut Rgb, (col, row): (usize, usize)) {
        if let Ok(profile_index) = get_profile_index(keyboard) {
            if row == 0 && col == 17 + profile_index as usize {
                *rgb = [u8::MAX; 3];
            }
        }
    }
}

#[memoize(Ignore: keyboard, TimeToLive: Duration::from_secs(1))]
pub fn get_profile_index(keyboard: &Keyboard) -> Result<u8, &'static str> {
    keyboard
        .send_command_with_args(Command::GetCurrentKeyboardProfileIndex, 0, 0, 0, 0)
        .map_err(|_| "Failed to get current keyboard profile index")
        .map(|response| response[5])
}
