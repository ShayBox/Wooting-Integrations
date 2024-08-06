use crate::Keyboard;

#[cfg(feature = "animation")]
mod animation;
#[cfg(feature = "hyprland")]
mod hyprland;
#[cfg(feature = "mangohud")]
mod mangohud;
#[cfg(feature = "wooting")]
mod wooting;

pub mod prelude;

pub trait Integration {
    fn color(&mut self, rgba: &mut [u8; 4], pos: (usize, usize), keyboard: &Keyboard);
}
