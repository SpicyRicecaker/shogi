use crate::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_debug).add_system(debug_system);
    }
}

fn spawn_debug() {}

fn debug_system() {}
