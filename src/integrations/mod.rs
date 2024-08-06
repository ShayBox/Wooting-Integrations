#[cfg(feature = "hyprland")]
mod hyprland;

#[cfg(feature = "rainbow")]
mod rainbow;

#[cfg(feature = "wooting")]
mod wooting;

use anyhow::Result;

use crate::Keyboard;

pub mod prelude;

pub trait Integration {
    /// # Errors
    fn next(&mut self, keyboard: &Keyboard) -> Result<()>;
    fn color(&mut self, rgba: &mut [u8; 4], pos: (usize, usize));
}
