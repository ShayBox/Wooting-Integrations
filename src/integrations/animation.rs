use std::time::Instant;

use colorgrad::{preset::SinebowGradient, Gradient};

use crate::{Keyboard, WOOTING_RGB_COLS, WOOTING_RGB_ROWS};

use super::Integration;

pub struct Animation {
    pub instant: Instant,
    pub gradient: SinebowGradient,
}

impl Default for Animation {
    #[must_use]
    fn default() -> Self {
        Self {
            instant: Instant::now(),
            gradient: SinebowGradient {},
        }
    }
}

impl Integration for Animation {
    fn color(&mut self, rgba: &mut [u8; 4], (col, row): (usize, usize), _: &Keyboard) {
        #[allow(clippy::cast_precision_loss)]
        let (x, y) = (
            col as f32 / (WOOTING_RGB_COLS - 1) as f32 / 2.0,
            row as f32 / (WOOTING_RGB_ROWS - 1) as f32 / 5.0,
        );

        let elapsed = self.instant.elapsed().as_secs_f32();
        let progress = (elapsed / -3.0) % 1.0;
        let position = (x + y + progress) % 1.0;
        let color = self.gradient.at(position);

        *rgba = color.to_rgba8();
    }
}
