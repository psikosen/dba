use bevy::prelude::*;
use bevy_shaman_core::resources::GameCalendar;

use crate::components::{MetabolicHistory, PrimeVessel};
use crate::events::VesselDecayProcessed;

/// Process Metabolic Decay for the Prime Vessel
/// Spirits absorbed over 800 days ago are fully digested and removed
pub fn process_metabolic_decay(
    mut query: Query<(&mut PrimeVessel, &mut MetabolicHistory)>,
    calendar: Res<GameCalendar>,
    mut decay_events: EventWriter<VesselDecayProcessed>,
) {
    for (mut vessel, mut history) in query.iter_mut() {
        // Skip if vessel is dormant
        if !vessel.is_active {
            continue;
        }

        let current_day = calendar.current_day();
        let spirits_before = history.spirit_count() as u32;
        let power_lost = history.process_decay(current_day);

        if power_lost > 0.0 {
            let spirits_decayed = spirits_before - history.spirit_count() as u32;

            // Reduce vessel power
            vessel.power_level = (vessel.power_level - power_lost).max(0.0);

            // Fire decay event
            decay_events.send(VesselDecayProcessed {
                power_lost,
                spirits_decayed,
                new_power: vessel.power_level,
            });

            // Log significant decay
            if power_lost > 100.0 {
                info!(
                    "Prime Vessel metabolized {} spirits ({:.0} power) - now at {:.0} power",
                    spirits_decayed, power_lost, vessel.power_level
                );
            }
        }
    }
}

/// Check if the Prime Vessel should downgrade its evolution tier
/// due to power loss from decay
pub fn check_tier_downgrade(mut query: Query<&mut PrimeVessel, Changed<PrimeVessel>>) {
    for mut vessel in query.iter_mut() {
        if vessel.evolution_tier == 0 {
            continue;
        }

        // Calculate the threshold for current tier (what was needed to reach it)
        let current_tier_threshold = 500.0 * (2.5_f32).powi((vessel.evolution_tier - 1) as i32);

        // If power drops below 50% of what was needed to reach current tier, downgrade
        if vessel.power_level < current_tier_threshold * 0.5 {
            vessel.evolution_tier = vessel.evolution_tier.saturating_sub(1);
            warn!(
                "Prime Vessel weakened! Tier downgrade to {} (power: {:.0})",
                vessel.evolution_tier, vessel.power_level
            );
        }
    }
}
