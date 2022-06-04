use bevy::prelude::{
    Res,
    Query,
    Text,
};

use crate::collision::{
    WallLocation,
    Side,
};

// This resource tracks the game's score
pub struct Scoreboard {
    pub left: usize,
    pub right: usize
}

pub fn update_scoreboard(scoreboard: Res<Scoreboard>, mut query: Query<(&mut Text, &Side)>) {
    for (mut text, side) in query.iter_mut()
    {
        text.sections[1].value = format!("{}", match side.0 {
            WallLocation::Left => scoreboard.left,
            WallLocation::Right => scoreboard.right,
            _ => 0,
        });
    }
}