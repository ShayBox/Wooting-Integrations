#[cfg(feature = "animation")]
mod animation;
#[cfg(feature = "hyprland")]
mod hyprland;
#[cfg(feature = "latency")]
mod latency;
#[cfg(feature = "mangohud")]
mod mangohud;
#[cfg(feature = "wooting")]
mod wooting;

pub mod prelude;

use crate::wooting::{Keyboard, Rgb};

pub trait Integration {
    fn color(&mut self, keyboard: &Keyboard, rgb: &mut Rgb, pos: (usize, usize));
}
