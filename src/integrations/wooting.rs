use anyhow::Result;

use crate::{Command, Keyboard};

use super::Integration;

#[derive(Default)]
pub struct Wooting(u8);

impl Integration for Wooting {
    fn next(&mut self, keyboard: &Keyboard) -> Result<()> {
        self.0 = keyboard.send_command(Command::GetCurrentKeyboardProfileIndex, 0, 0, 0, 0)?[5];

        Ok(())
    }

    fn color(&mut self, rgba: &mut [u8; 4], (col, row): (usize, usize)) {
        if row == 0 && col == 17 + self.0 as usize {
            *rgba = [u8::MAX; 4];
        }
    }
}
