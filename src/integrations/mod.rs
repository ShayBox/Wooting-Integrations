mod hyprland;
mod rainbow;

use anyhow::Result;

pub mod prelude;

pub trait Integration {
    /// # Errors
    fn next(&mut self) -> Result<()>;
    fn color(&mut self, rgba: &mut [u8; 4], pos: (usize, usize));
}
