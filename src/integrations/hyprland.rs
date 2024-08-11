use std::{collections::HashMap, time::Duration};

use hyprland::{
    data::{Workspace, Workspaces},
    shared::{HyprData, HyprDataActive},
};
use memoize::memoize;

use crate::{Keyboard, Rgb};

use super::Integration;

#[derive(Clone, Default)]
pub struct Hyprland {
    active: HashMap<i128, i32>,
}

impl Integration for Hyprland {
    fn color(&mut self, _: &Keyboard, rgba: &mut Rgb, pos: (usize, usize)) {
        self.active = get_cached_active(self.active.clone());

        for id in get_cached_workspaces() {
            if pos == get_pos_from_workspace(id) {
                *rgba = [u8::MIN; 3];
            }
        }

        for id in self.active.values() {
            if pos == get_pos_from_workspace(*id) {
                *rgba = [u8::MAX; 3];
            }
        }
    }
}

#[must_use]
pub const fn get_pos_from_workspace(id: i32) -> (usize, usize) {
    match id {
        1 => (17, 4),
        2 => (18, 4),
        3 => (19, 4),
        4 => (17, 3),
        5 => (18, 3),
        6 => (19, 3),
        7 => (17, 2),
        8 => (18, 2),
        9 => (19, 2),
        _ => (0, 0),
    }
}

#[memoize(Ignore: active, TimeToLive: Duration::from_millis(100))]
pub fn get_cached_active(mut active: HashMap<i128, i32>) -> HashMap<i128, i32> {
    let workspace = Workspace::get_active().expect("Failed to get active");
    active.insert(workspace.monitor_id, workspace.id);
    active
}

#[memoize(TimeToLive: Duration::from_secs(1))]
pub fn get_cached_workspaces() -> Vec<i32> {
    Workspaces::get()
        .expect("Failed to get workspaces")
        .into_iter()
        .map(|workspace| workspace.id)
        .filter(|id| *id > 0)
        .collect()
}
