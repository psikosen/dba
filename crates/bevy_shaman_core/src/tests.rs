#[cfg(test)]
mod core_tests {
    use super::super::components::*;

    // ============================================================================
    // GRID POSITION TESTS
    // ============================================================================

    #[test]
    fn test_grid_position_new() {
        let pos = GridPosition::new(5, 10);
        assert_eq!(pos.x, 5);
        assert_eq!(pos.y, 10);
    }

    #[test]
    fn test_grid_position_distance() {
        let pos1 = GridPosition::new(0, 0);
        let pos2 = GridPosition::new(3, 4);

        let dist = pos1.distance(&pos2);
        assert_eq!(dist, 7); // Manhattan distance: |3| + |4| = 7
    }

    #[test]
    fn test_grid_position_distance_same_point() {
        let pos = GridPosition::new(5, 5);
        assert_eq!(pos.distance(&pos), 0);
    }

    #[test]
    fn test_grid_position_distance_negative() {
        let pos1 = GridPosition::new(0, 0);
        let pos2 = GridPosition::new(-3, -4);

        let dist = pos1.distance(&pos2);
        assert_eq!(dist, 7);
    }

    // ============================================================================
    // HEALTH TESTS
    // ============================================================================

    #[test]
    fn test_health_new() {
        let health = Health::new(100.0);
        assert_eq!(health.current, 100.0);
        assert_eq!(health.max, 100.0);
    }

    #[test]
    fn test_health_damage() {
        let mut health = Health::new(100.0);
        health.damage(30.0);
        assert_eq!(health.current, 70.0);
    }

    #[test]
    fn test_health_damage_below_zero() {
        let mut health = Health::new(50.0);
        health.damage(100.0);
        assert_eq!(health.current, 0.0);
    }

    #[test]
    fn test_health_heal() {
        let mut health = Health::new(100.0);
        health.damage(50.0);
        health.heal(30.0);
        assert_eq!(health.current, 80.0);
    }

    #[test]
    fn test_health_heal_above_max() {
        let mut health = Health::new(100.0);
        health.damage(20.0);
        health.heal(50.0);
        assert_eq!(health.current, 100.0);
    }

    #[test]
    fn test_health_is_dead() {
        let mut health = Health::new(100.0);
        assert!(!health.is_dead());

        health.damage(100.0);
        assert!(health.is_dead());
    }

    // ============================================================================
    // SPIRIT TESTS
    // ============================================================================

    #[test]
    fn test_spirit_default() {
        let spirit = Spirit::default();
        assert_eq!(spirit.current, 100.0);
        assert_eq!(spirit.max, 100.0);
        assert_eq!(spirit.regen_rate, 5.0);
    }

    #[test]
    fn test_spirit_custom() {
        let spirit = Spirit {
            current: 50.0,
            max: 100.0,
            regen_rate: 2.0,
        };
        assert_eq!(spirit.current, 50.0);
        assert_eq!(spirit.max, 100.0);
    }

    #[test]
    fn test_spirit_heal() {
        let mut spirit = Spirit {
            current: 50.0,
            max: 100.0,
            regen_rate: 1.0,
        };
        spirit.heal(30.0);
        assert_eq!(spirit.current, 80.0);
    }

    #[test]
    fn test_spirit_heal_above_max() {
        let mut spirit = Spirit {
            current: 80.0,
            max: 100.0,
            regen_rate: 1.0,
        };
        spirit.heal(50.0);
        assert_eq!(spirit.current, 100.0);
    }


    // ============================================================================
    // STAMINA TESTS
    // ============================================================================

    #[test]
    fn test_stamina_default() {
        let stamina = Stamina::default();
        assert_eq!(stamina.current, 100.0);
        assert_eq!(stamina.max, 100.0);
        assert_eq!(stamina.regen_rate, 10.0);
    }

    #[test]
    fn test_stamina_custom() {
        let stamina = Stamina {
            current: 50.0,
            max: 150.0,
            regen_rate: 5.0,
        };
        assert_eq!(stamina.current, 50.0);
        assert_eq!(stamina.max, 150.0);
    }

    // ============================================================================
    // EDGE CASE TESTS - Finding Bugs!
    // ============================================================================

    #[test]
    fn test_health_zero_max() {
        let health = Health::new(0.0);
        assert_eq!(health.current, 0.0);
        assert_eq!(health.max, 0.0);
        assert!(health.is_dead());
    }

    #[test]
    fn test_health_negative_damage() {
        let mut health = Health::new(100.0);
        health.damage(-10.0);
        // Should either do nothing or heal
        assert!(health.current >= 100.0);
    }

    #[test]
    fn test_spirit_zero_regen() {
        let spirit = Spirit {
            current: 50.0,
            max: 100.0,
            regen_rate: 0.0,
        };
        assert_eq!(spirit.regen_rate, 0.0);
    }

    #[test]
    fn test_stamina_max_values() {
        let stamina = Stamina {
            current: 200.0,
            max: 200.0,
            regen_rate: 15.0,
        };
        assert_eq!(stamina.current, stamina.max);
    }

    #[test]
    fn test_grid_position_large_distances() {
        let pos1 = GridPosition::new(0, 0);
        let pos2 = GridPosition::new(1000, 1000);

        let dist = pos1.distance(&pos2);
        assert_eq!(dist, 2000);
    }

    #[test]
    fn test_grid_position_negative_coords() {
        let pos1 = GridPosition::new(-100, -200);
        let pos2 = GridPosition::new(100, 200);

        let dist = pos1.distance(&pos2);
        assert_eq!(dist, 600);
    }

    #[test]
    fn test_health_fractional_values() {
        let mut health = Health::new(100.5);
        health.damage(20.3);
        health.heal(10.1);

        assert!((health.current - 90.3).abs() < 0.001);
    }

    #[test]
    fn test_spirit_values() {
        let spirit = Spirit {
            current: 50.0,
            max: 100.0,
            regen_rate: 5.0,
        };
        assert!(spirit.current < spirit.max);
    }

    #[test]
    fn test_player_marker() {
        // Just ensure Player marker component exists
        let _player = Player;
    }

    #[test]
    fn test_movement_queue() {
        let mut queue = MovementQueue::default();
        assert_eq!(queue.commands.len(), 0);

        queue.commands.push(MovementCommand::Move(bevy::prelude::IVec2::new(1, 0)));
        assert_eq!(queue.commands.len(), 1);
    }

    #[test]
    fn test_sprite_animation() {
        let anim = SpriteAnimation::new(0.1, 4, true);
        assert_eq!(anim.frame_count, 4);
        assert_eq!(anim.current_frame, 0);
        assert!(anim.looping);
    }
}
