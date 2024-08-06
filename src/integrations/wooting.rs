use std::time::Duration;

use memoize::memoize;

use crate::{Command, Keyboard, Rgb};

use super::Integration;

#[derive(Default)]
pub struct Wooting(());

impl Integration for Wooting {
    fn color(&mut self, keyboard: &Keyboard, rgb: &mut Rgb, (col, row): (usize, usize)) {
        let profile_index = get_profile_index(keyboard);
        if row == 0 && col == 17 + profile_index as usize {
            *rgb = [u8::MAX; 3];
        }
    }
}

#[memoize(Ignore: keyboard, TimeToLive: Duration::from_secs(1))]
pub fn get_profile_index(keyboard: &Keyboard) -> u8 {
    keyboard
        .send_command(Command::GetCurrentKeyboardProfileIndex, 0, 0, 0, 0)
        .expect("Failed to get current keyboard profile index")[5]
}
