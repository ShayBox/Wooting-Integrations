use std::{process::Command, time::Duration};

use memoize::memoize;

use crate::wooting::{Keyboard, Rgb};

use super::Integration;

macro_rules! run {
    ($($tt:tt)*) => {
        String::from_utf8(
            Command::new("sh")
                .arg("-c")
                .arg(format!($($tt)*))
                .output()
                .expect("Failed to run command")
                .stdout,
        )
        .expect("Failed to convert string")
    };
}

#[derive(Default)]
pub struct Mangohud(());

impl Integration for Mangohud {
    fn color(&mut self, _: &Keyboard, rgb: &mut Rgb, pos: (usize, usize)) {
        if pos != (0, 0) {
            return;
        }

        *rgb = match get_cached_framerate() {
            f64::MIN..25.0 => [178, 34, 34], // Red
            25.0..55.0 => [253, 253, 9],     // Yellow
            55.0..f64::MAX => [57, 249, 0],  // Green
            _ => return,
        };
    }
}

#[memoize(TimeToLive: Duration::from_millis(500))]
pub fn get_cached_framerate() -> f64 {
    const DIR: &str = "/tmp/mangohud";
    let file = run!("ls -t {DIR} | head -1");
    let fps = run!("tail -1 {DIR}/{} | cut -d, -f1", file.trim());
    fps.trim().parse::<f64>().unwrap_or_default()
}
