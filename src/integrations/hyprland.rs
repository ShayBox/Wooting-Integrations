use std::collections::HashMap;

use anyhow::Result;
use hyprland::{data::Workspace, shared::HyprDataActive};

use super::Integration;

#[derive(Default)]
pub struct Hyprland {
    workspaces: HashMap<i128, i32>,
}

impl Integration for Hyprland {
    fn next(&mut self) -> Result<()> {
        let workspace = Workspace::get_active()?;
        self.workspaces.insert(workspace.monitor_id, workspace.id);

        Ok(())
    }

    fn color(&mut self, rgba: &mut [u8; 4], pos: (usize, usize)) {
        for workspace_id in self.workspaces.values() {
            let key = match workspace_id {
                1 => (4, 17),
                2 => (4, 18),
                3 => (4, 19),
                4 => (3, 17),
                5 => (3, 18),
                6 => (3, 19),
                7 => (2, 17),
                8 => (2, 18),
                9 => (2, 19),
                _ => (usize::MAX, usize::MAX),
            };

            if key == pos {
                *rgba = [u8::MAX; 4];
            }
        }
    }
}
