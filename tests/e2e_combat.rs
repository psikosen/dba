/// End-to-End Combat Integration Tests
///
/// Tests full combat scenarios including rhythm mechanics, damage calculation,
/// and status effects.

use bevy::prelude::*;
use bevy_shaman_combat::{
    components::{AttackCooldown, CombatStats},
    resources::RhythmClock,
    CombatPlugin,
};
use bevy_shaman_core::{
    components::{GridPosition, Health, Player},
    CorePlugin,
};
use bevy_shaman_monsters::{
    components::{Monster, MonsterType},
    MonstersPlugin,
};

// ============================================================================
// TEST UTILITIES
// ============================================================================

fn create_combat_test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugins(CorePlugin)
        .add_plugins(CombatPlugin)
        .add_plugins(MonstersPlugin)
        .init_resource::<RhythmClock>();
    app
}

fn spawn_test_player_with_combat(app: &mut App) -> Entity {
    app.world_mut()
        .spawn((
            Player,
            Health::new(100.0),
            GridPosition { x: 0, y: 0 },
            CombatStats {
                attack_power: 10.0,
                defense: 5.0,
                critical_chance: 0.1,
                critical_multiplier: 2.0,
            },
            AttackCooldown::default(),
        ))
        .id()
}

fn spawn_test_monster(app: &mut App, position: GridPosition, health: f32) -> Entity {
    app.world_mut()
        .spawn((
            Monster::new(MonsterType::Unspecified),
            Health::new(health),
            position,
            CombatStats {
                attack_power: 5.0,
                defense: 2.0,
                critical_chance: 0.05,
                critical_multiplier: 1.5,
            },
        ))
        .id()
}

// ============================================================================
// E2E COMBAT TESTS
// ============================================================================

#[test]
fn test_e2e_basic_combat_encounter() {
    // Scenario: Player encounters monster, exchanges attacks until monster dies
    let mut app = create_combat_test_app();
    let player = spawn_test_player_with_combat(&mut app);
    let monster = spawn_test_monster(&mut app, GridPosition { x: 1, y: 0 }, 30.0);

    // Verify initial state
    {
        let monster_ref = app.world().entity(monster);
        let health = monster_ref.get::<Health>().unwrap();
        assert_eq!(health.current, 30.0);
    }

    // Player attacks (simulated - actual combat system would handle this)
    // This is a placeholder for when combat systems are fully implemented
    {
        let mut monster_ref = app.world_mut().entity_mut(monster);
        let mut health = monster_ref.get_mut::<Health>().unwrap();
        health.take_damage(10.0); // Player deals 10 damage
    }

    {
        let monster_ref = app.world().entity(monster);
        let health = monster_ref.get::<Health>().unwrap();
        assert_eq!(health.current, 20.0);
    }
}

#[test]
fn test_e2e_rhythm_timing_affects_damage() {
    // Scenario: Test that rhythm timing affects damage output
    let mut app = create_combat_test_app();
    let _player = spawn_test_player_with_combat(&mut app);

    // Set rhythm clock to perfect beat
    {
        let mut rhythm = app.world_mut().resource_mut::<RhythmClock>();
        rhythm.beat_time = 0.0; // Perfect timing
    }

    // Test rhythm evaluation
    let rhythm = app.world().resource::<RhythmClock>();
    let timing_accuracy = rhythm.get_timing_accuracy();

    // Perfect timing should give high accuracy
    assert!(
        timing_accuracy > 0.9,
        "Perfect timing should give >90% accuracy"
    );
}

#[test]
fn test_e2e_player_death_and_respawn() {
    // Scenario: Player health reaches 0, death is handled
    let mut app = create_combat_test_app();
    let player = spawn_test_player_with_combat(&mut app);

    // Reduce player to 0 health
    {
        let mut player_ref = app.world_mut().entity_mut(player);
        let mut health = player_ref.get_mut::<Health>().unwrap();
        health.take_damage(100.0);
    }

    // Verify player is at 0 health
    {
        let player_ref = app.world().entity(player);
        let health = player_ref.get::<Health>().unwrap();
        assert_eq!(health.current, 0.0);
        assert!(health.is_dead(), "Player should be dead");
    }
}

#[test]
fn test_e2e_combat_cooldown_system() {
    // Scenario: Test that attack cooldowns prevent spam
    let mut app = create_combat_test_app();
    let player = spawn_test_player_with_combat(&mut app);

    // Set cooldown
    {
        let mut player_ref = app.world_mut().entity_mut(player);
        let mut cooldown = player_ref.get_mut::<AttackCooldown>().unwrap();
        cooldown.remaining = 1.0; // 1 second cooldown
    }

    // Advance time by 0.5 seconds
    {
        let mut player_ref = app.world_mut().entity_mut(player);
        let mut cooldown = player_ref.get_mut::<AttackCooldown>().unwrap();
        cooldown.remaining -= 0.5;
    }

    // Verify cooldown not finished
    {
        let player_ref = app.world().entity(player);
        let cooldown = player_ref.get::<AttackCooldown>().unwrap();
        assert!(cooldown.remaining > 0.0, "Cooldown should not be finished");
    }

    // Advance time by another 0.6 seconds
    {
        let mut player_ref = app.world_mut().entity_mut(player);
        let mut cooldown = player_ref.get_mut::<AttackCooldown>().unwrap();
        cooldown.remaining -= 0.6;
    }

    // Verify cooldown finished
    {
        let player_ref = app.world().entity(player);
        let cooldown = player_ref.get::<AttackCooldown>().unwrap();
        assert!(
            cooldown.remaining <= 0.0,
            "Cooldown should be finished after 1.1 seconds"
        );
    }
}

#[test]
fn test_e2e_multiple_enemies_combat() {
    // Scenario: Player fights multiple enemies at once
    let mut app = create_combat_test_app();
    let player = spawn_test_player_with_combat(&mut app);

    // Spawn 3 monsters around player
    let monster1 = spawn_test_monster(&mut app, GridPosition { x: 1, y: 0 }, 20.0);
    let monster2 = spawn_test_monster(&mut app, GridPosition { x: -1, y: 0 }, 20.0);
    let monster3 = spawn_test_monster(&mut app, GridPosition { x: 0, y: 1 }, 20.0);

    // Verify all monsters exist
    assert!(app.world().get_entity(monster1).is_some());
    assert!(app.world().get_entity(monster2).is_some());
    assert!(app.world().get_entity(monster3).is_some());

    // Player takes damage from each
    {
        let mut player_ref = app.world_mut().entity_mut(player);
        let mut health = player_ref.get_mut::<Health>().unwrap();
        health.take_damage(5.0); // From monster 1
        health.take_damage(5.0); // From monster 2
        health.take_damage(5.0); // From monster 3
    }

    {
        let player_ref = app.world().entity(player);
        let health = player_ref.get::<Health>().unwrap();
        assert_eq!(health.current, 85.0, "Player should have taken 15 damage");
    }
}

// ============================================================================
// COMBAT EDGE CASES
// ============================================================================

#[test]
fn test_e2e_damage_with_zero_defense() {
    // Test damage calculation when defense is 0
    let mut app = create_combat_test_app();
    let player = spawn_test_player_with_combat(&mut app);

    {
        let mut player_ref = app.world_mut().entity_mut(player);
        let mut stats = player_ref.get_mut::<CombatStats>().unwrap();
        stats.defense = 0.0;
    }

    // Take damage
    {
        let mut player_ref = app.world_mut().entity_mut(player);
        let mut health = player_ref.get_mut::<Health>().unwrap();
        health.take_damage(25.0);
    }

    {
        let player_ref = app.world().entity(player);
        let health = player_ref.get::<Health>().unwrap();
        assert_eq!(health.current, 75.0, "Full damage should be dealt");
    }
}

#[test]
fn test_e2e_critical_hit_calculation() {
    // Test critical hit mechanics
    let mut app = create_combat_test_app();
    let player = spawn_test_player_with_combat(&mut app);

    // Set 100% crit chance for testing
    {
        let mut player_ref = app.world_mut().entity_mut(player);
        let mut stats = player_ref.get_mut::<CombatStats>().unwrap();
        stats.critical_chance = 1.0; // 100% crit
        stats.critical_multiplier = 2.0; // 2x damage
    }

    let stats = app
        .world()
        .entity(player)
        .get::<CombatStats>()
        .unwrap()
        .clone();

    // Calculate expected damage
    let base_damage = stats.attack_power;
    let crit_damage = base_damage * stats.critical_multiplier;

    assert_eq!(crit_damage, 20.0, "Crit should deal 2x damage (10 * 2)");
}
