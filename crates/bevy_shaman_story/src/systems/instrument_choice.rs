use bevy::prelude::*;
use bevy_shaman_core::components::{Player, Stamina};
use crate::resources::InstrumentChoice;

/// Applies instrument-specific modifiers to player stats and abilities
pub fn apply_instrument_modifiers(
    instrument: Res<InstrumentChoice>,
    mut players: Query<&mut Stamina, With<Player>>,
) {
    if *instrument == InstrumentChoice::NotChosen {
        return;
    }

    // Example: modify stamina regen based on instrument
    for mut stamina in players.iter_mut() {
        stamina.regen_rate = match *instrument {
            InstrumentChoice::Kora => 12.0,  // Better stamina management
            InstrumentChoice::Ngoni => 8.0,  // Worse stamina management
            InstrumentChoice::NotChosen => 10.0,
        };
    }
}
