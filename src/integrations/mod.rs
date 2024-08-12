use crate::{Keyboard, Rgb};

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

pub trait Integration {
    fn color(&mut self, keyboard: &Keyboard, rgb: &mut Rgb, pos: (usize, usize));
}
