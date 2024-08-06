use std::time::Instant;

use colorgrad::{preset::SinebowGradient, Gradient};

use crate::{Keyboard, Rgb, WOOTING_RGB_COLS, WOOTING_RGB_ROWS};

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
    fn color(&mut self, _: &Keyboard, rgb: &mut Rgb, (col, row): (usize, usize)) {
        #[allow(clippy::cast_precision_loss)]
        let (x, y) = (
            col as f32 / (WOOTING_RGB_COLS - 1) as f32 / 2.0,
            row as f32 / (WOOTING_RGB_ROWS - 1) as f32 / 5.0,
        );

        let elapsed = self.instant.elapsed().as_secs_f32();
        let progress = (elapsed / -3.0) % 1.0;
        let position = (x + y + progress) % 1.0;
        let color = self.gradient.at(position);
        let rgba = color.to_rgba8();

        rgb.copy_from_slice(&rgba[0..3]);
    }
}
