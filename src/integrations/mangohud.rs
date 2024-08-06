use std::{process::Command, time::Duration};

use memoize::memoize;

use crate::Keyboard;

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
    fn color(&mut self, rgba: &mut [u8; 4], pos: (usize, usize), _: &Keyboard) {
        if pos != (0, 0) {
            return;
        }

        *rgba = match get_cached_framerate() {
            0.0..24.0 => [255, 0, 0, 0],
            24.0..48.0 => [255, 255, 0, 0],
            48.0..96.0 => [0, 255, 0, 0],
            _ => return,
        };
    }
}

#[memoize(TimeToLive: Duration::from_secs(1))]
pub fn get_cached_framerate() -> f64 {
    const DIR: &str = "/tmp/mangohud";
    let file = run!("ls -t {DIR} | head -n 1");
    let line = run!("tail -n 1 {DIR}/{file}");
    let fps = run!("echo '{line}' | cut -d',' -f1");
    fps.trim().parse::<f64>().unwrap_or_default()
}
