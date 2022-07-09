use bevy::prelude::*;

use crate::*;
use crate::mouse::*;

pub struct ReservePlugin;

impl Plugin for ReservePlugin {
    fn build(&self, app: &mut App) {
        println!("hello world");
        // app.add_startup_system(spawn_reserve).add_system_to_stage(CoreStage::Last, reserve_system);
    }
}

// we should actually just fully delete the position entity instead of trying to mutate it, and
// have every reserve be unique
// each reserve has 
// - sprite
// - transform
// - count
// - piece_type
fn reserve_system(
    mut ev_take: EventReader<TakeEvent>,
    mut reserve_query: Query<(&mut Reserve, &PieceType, &Player, &Children)>,
    reserve_query_child: Query<&mut Sprite>
) {
    for e in ev_take.iter() {
        if let Some((mut reserve, _, _, child)) = reserve_query.iter_mut().find(|(_, &pt, &o, _)| pt == e.piece_type && o == e.taker) {
            reserve.quantity += 1;
            // update sprite of child
        }
    }
}
