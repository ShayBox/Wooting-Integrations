use std::collections::HashMap;

use anyhow::Result;
use hyprland::{
    data::{Workspace, Workspaces},
    shared::{HyprData, HyprDataActive},
};

use crate::Keyboard;

use super::Integration;

#[derive(Default)]
pub struct Hyprland {
    active: HashMap<i128, i32>,
    workspaces: Vec<i32>,
}

impl Integration for Hyprland {
    fn next(&mut self, _keyboard: &Keyboard) -> Result<()> {
        let active = Workspace::get_active()?;
        self.active.insert(active.monitor_id, active.id);
        self.workspaces = Workspaces::get()?
            .into_iter()
            .map(|workspace| workspace.id)
            .filter(|id| *id > 0)
            .collect();

        Ok(())
    }

    fn color(&mut self, rgba: &mut [u8; 4], pos: (usize, usize)) {
        for id in &self.workspaces {
            if pos == Self::get_pos_from_workspace(id) {
                *rgba = [u8::MIN; 4];
            }
        }

        for id in self.active.values() {
            if pos == Self::get_pos_from_workspace(id) {
                *rgba = [u8::MAX; 4];
            }
        }
    }
}

impl Hyprland {
    #[must_use]
    pub const fn get_pos_from_workspace(id: &i32) -> (usize, usize) {
        match id {
            1 => (4, 17),
            2 => (4, 18),
            3 => (4, 19),
            4 => (3, 17),
            5 => (3, 18),
            6 => (3, 19),
            7 => (2, 17),
            8 => (2, 18),
            9 => (2, 19),
            _ => (0, 0),
        }
    }
}
