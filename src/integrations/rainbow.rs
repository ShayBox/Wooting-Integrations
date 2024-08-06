use std::time::Instant;

use anyhow::Result;
use colorgrad::{preset::SinebowGradient, Gradient};

use crate::{WOOTING_RGB_COLS, WOOTING_RGB_ROWS};

use super::Integration;

pub struct Rainbow {
    pub instant: Instant,
    pub gradient: SinebowGradient,
    progress: f32,
}

impl Default for Rainbow {
    #[must_use]
    fn default() -> Self {
        Self {
            instant: Instant::now(),
            gradient: SinebowGradient {},
            progress: 0.0,
        }
    }
}

impl Integration for Rainbow {
    fn next(&mut self) -> Result<()> {
        let elapsed = self.instant.elapsed().as_secs_f32();
        self.progress = (elapsed / -3.0) % 1.0;

        Ok(())
    }

    fn color(&mut self, rgba: &mut [u8; 4], (col, row): (usize, usize)) {
        #[allow(clippy::cast_precision_loss)]
        let (x, y) = (
            col as f32 / (WOOTING_RGB_COLS - 1) as f32 / 2.0,
            row as f32 / (WOOTING_RGB_ROWS - 1) as f32 / 5.0,
        );

        let progress = (x + y + self.progress) % 1.0;
        let color = self.gradient.at(progress);

        *rgba = color.to_rgba8();
    }
}
