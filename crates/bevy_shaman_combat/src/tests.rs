#[cfg(test)]
mod combat_tests {
    use super::super::components::*;
    use bevy_shaman_core::resources::TimingQuality;

    // ============================================================================
    // BLOOD LUST TESTS
    // ============================================================================

    #[test]
    fn test_blood_lust_default() {
        let blood_lust = BloodLust::default();
        assert_eq!(blood_lust.current, 0.0);
        assert_eq!(blood_lust.threshold, 70.0);
        assert_eq!(blood_lust.decay_rate, 5.0);
        assert!(!blood_lust.is_corrupting());
    }

    #[test]
    fn test_blood_lust_combat_gain_easy() {
        let mut blood_lust = BloodLust::default();
        blood_lust.add_from_combat(50.0, false, CombatDifficulty::Easy);
        assert_eq!(blood_lust.current, 2.0);
    }

    #[test]
    fn test_blood_lust_combat_gain_normal() {
        let mut blood_lust = BloodLust::default();
        blood_lust.add_from_combat(75.0, false, CombatDifficulty::Normal);
        assert_eq!(blood_lust.current, 5.0);
    }

    #[test]
    fn test_blood_lust_combat_gain_hard() {
        let mut blood_lust = BloodLust::default();
        blood_lust.add_from_combat(100.0, false, CombatDifficulty::Hard);
        assert_eq!(blood_lust.current, 10.0);
    }

    #[test]
    fn test_blood_lust_combat_gain_boss() {
        let mut blood_lust = BloodLust::default();
        blood_lust.add_from_combat(200.0, false, CombatDifficulty::Boss);
        assert_eq!(blood_lust.current, 15.0);
    }

    #[test]
    fn test_blood_lust_overkill_doubles_gain() {
        let mut blood_lust = BloodLust::default();
        blood_lust.add_from_combat(100.0, true, CombatDifficulty::Normal);
        assert_eq!(blood_lust.current, 10.0); // 5.0 * 2.0
    }

    #[test]
    fn test_blood_lust_caps_at_100() {
        let mut blood_lust = BloodLust::default();
        blood_lust.current = 95.0;
        blood_lust.add_from_combat(200.0, true, CombatDifficulty::Boss);
        assert_eq!(blood_lust.current, 100.0);
    }

    #[test]
    fn test_blood_lust_is_corrupting() {
        let mut blood_lust = BloodLust::default();
        assert!(!blood_lust.is_corrupting());

        blood_lust.current = 70.0;
        assert!(blood_lust.is_corrupting());

        blood_lust.current = 69.9;
        assert!(!blood_lust.is_corrupting());
    }

    #[test]
    fn test_blood_lust_reduce_with_music() {
        let mut blood_lust = BloodLust::default();
        blood_lust.current = 50.0;
        blood_lust.reduce_with_music(20.0);
        assert_eq!(blood_lust.current, 30.0);
    }

    #[test]
    fn test_blood_lust_reduce_with_plant() {
        let mut blood_lust = BloodLust::default();
        blood_lust.current = 40.0;
        blood_lust.reduce_with_plant(15.0);
        assert_eq!(blood_lust.current, 25.0);
    }

    #[test]
    fn test_blood_lust_reduce_with_food() {
        let mut blood_lust = BloodLust::default();
        blood_lust.current = 30.0;
        blood_lust.reduce_with_food(10.0);
        assert_eq!(blood_lust.current, 20.0);
    }

    #[test]
    fn test_blood_lust_reduce_floors_at_zero() {
        let mut blood_lust = BloodLust::default();
        blood_lust.current = 5.0;
        blood_lust.reduce_with_music(10.0);
        assert_eq!(blood_lust.current, 0.0);
    }

    // ============================================================================
    // WEAPON TYPE TESTS
    // ============================================================================

    #[test]
    fn test_weapon_base_damage() {
        assert_eq!(WeaponType::Crossbow.base_damage(), 15.0);
        assert_eq!(WeaponType::Mambele.base_damage(), 12.0);
        assert_eq!(WeaponType::Staff.base_damage(), 10.0);
    }

    #[test]
    fn test_weapon_spirit_cost() {
        assert_eq!(WeaponType::Crossbow.spirit_cost(), 0.0);
        assert_eq!(WeaponType::Mambele.spirit_cost(), 5.0);
        assert_eq!(WeaponType::Staff.spirit_cost(), 10.0);
    }

    #[test]
    fn test_weapon_can_apply_poison() {
        assert!(!WeaponType::Crossbow.can_apply_poison());
        assert!(WeaponType::Mambele.can_apply_poison());
        assert!(!WeaponType::Staff.can_apply_poison());
    }

    #[test]
    fn test_weapon_can_cast_spells() {
        assert!(!WeaponType::Crossbow.can_cast_spells());
        assert!(!WeaponType::Mambele.can_cast_spells());
        assert!(WeaponType::Staff.can_cast_spells());
    }

    #[test]
    fn test_equipped_weapon_default() {
        let weapon = EquippedWeapon::default();
        assert_eq!(weapon.weapon_type, WeaponType::Staff);
        assert_eq!(weapon.durability, 100.0);
        assert_eq!(weapon.max_durability, 100.0);
        assert_eq!(weapon.damage_multiplier, 1.0);
    }

    // ============================================================================
    // RHYTHM COMBO TESTS
    // ============================================================================

    #[test]
    fn test_rhythm_combo_new() {
        let combo = RhythmCombo::new(5, 2.0);
        assert_eq!(combo.max_combo_length, 5);
        assert_eq!(combo.combo_window, 2.0);
        assert_eq!(combo.current_combo.len(), 0);
    }

    #[test]
    fn test_rhythm_combo_add_input() {
        let mut combo = RhythmCombo::new(5, 2.0);
        combo.add_input(ComboInput::Light, 0.0);
        assert_eq!(combo.current_combo.len(), 1);
        assert_eq!(combo.current_combo[0], ComboInput::Light);
    }

    #[test]
    fn test_rhythm_combo_resets_outside_window() {
        let mut combo = RhythmCombo::new(5, 2.0);
        combo.add_input(ComboInput::Light, 0.0);
        combo.add_input(ComboInput::Heavy, 3.0); // Outside 2.0 second window
        assert_eq!(combo.current_combo.len(), 1);
        assert_eq!(combo.current_combo[0], ComboInput::Heavy);
    }

    #[test]
    fn test_rhythm_combo_trims_to_max_length() {
        let mut combo = RhythmCombo::new(3, 2.0);
        combo.add_input(ComboInput::Light, 0.0);
        combo.add_input(ComboInput::Light, 0.1);
        combo.add_input(ComboInput::Heavy, 0.2);
        combo.add_input(ComboInput::Magic, 0.3);

        assert_eq!(combo.current_combo.len(), 3);
        assert_eq!(combo.current_combo[0], ComboInput::Light);
        assert_eq!(combo.current_combo[1], ComboInput::Heavy);
        assert_eq!(combo.current_combo[2], ComboInput::Magic);
    }

    #[test]
    fn test_rhythm_combo_flurry_finisher() {
        let mut combo = RhythmCombo::new(5, 2.0);
        combo.add_input(ComboInput::Light, 0.0);
        combo.add_input(ComboInput::Light, 0.1);
        combo.add_input(ComboInput::Heavy, 0.2);

        let special = combo.check_special();
        assert_eq!(special, Some(SpecialMove::FlurryFinisher));
    }

    #[test]
    fn test_rhythm_combo_perfect_cast() {
        let mut combo = RhythmCombo::new(5, 2.0);
        combo.add_input(ComboInput::Perfect, 0.0);
        combo.add_input(ComboInput::Perfect, 0.1);
        combo.add_input(ComboInput::Magic, 0.2);

        let special = combo.check_special();
        assert_eq!(special, Some(SpecialMove::PerfectCast));
    }

    #[test]
    fn test_rhythm_combo_spirit_strike() {
        let mut combo = RhythmCombo::new(5, 2.0);
        combo.add_input(ComboInput::Heavy, 0.0);
        combo.add_input(ComboInput::Magic, 0.1);
        combo.add_input(ComboInput::Heavy, 0.2);

        let special = combo.check_special();
        assert_eq!(special, Some(SpecialMove::SpiritStrike));
    }

    #[test]
    fn test_rhythm_combo_no_special_when_short() {
        let mut combo = RhythmCombo::new(5, 2.0);
        combo.add_input(ComboInput::Light, 0.0);
        combo.add_input(ComboInput::Light, 0.1);

        let special = combo.check_special();
        assert_eq!(special, None);
    }

    #[test]
    fn test_rhythm_combo_reset() {
        let mut combo = RhythmCombo::new(5, 2.0);
        combo.add_input(ComboInput::Light, 0.0);
        combo.add_input(ComboInput::Heavy, 0.1);
        combo.reset();

        assert_eq!(combo.current_combo.len(), 0);
    }

    // ============================================================================
    // WHEEL OUTCOME TESTS
    // ============================================================================

    #[test]
    fn test_wheel_outcome_random_returns_valid() {
        // Test that random returns one of the four valid outcomes
        for _ in 0..100 {
            let outcome = WheelOutcome::random();
            match outcome {
                WheelOutcome::CriticalHit |
                WheelOutcome::DoubleSpellDamage |
                WheelOutcome::SelfCorruption |
                WheelOutcome::SpiritCorruption => {},
            }
        }
    }

    #[test]
    fn test_combat_wheel_default() {
        let wheel = CombatWheel::default();
        assert_eq!(wheel.trigger_chance, 0.18);
        assert_eq!(wheel.last_trigger, 0.0);
        assert_eq!(wheel.cooldown, 2.0);
    }

    // ============================================================================
    // MONSTER CONTROL TESTS
    // ============================================================================

    #[test]
    fn test_monster_control_default() {
        let control = MonsterControl::default();
        assert_eq!(control.controlled_monster, None);
        assert_eq!(control.control_duration, 0.0);
        assert_eq!(control.control_strength, 0.5);
        assert!(!control.can_use_abilities);
    }

    // ============================================================================
    // STATUS EFFECT TESTS
    // ============================================================================

    #[test]
    fn test_status_effect_types() {
        use StatusEffectType::*;
        let types = vec![Burn, Poison, SpiritualPoison, Stun, Slow, Purifying];

        for effect_type in types {
            let effect = StatusEffect {
                effect_type,
                duration: 5.0,
                strength: 10.0,
            };
            assert_eq!(effect.duration, 5.0);
            assert_eq!(effect.strength, 10.0);
        }
    }

    #[test]
    fn test_status_effects_default() {
        let effects = StatusEffects::default();
        assert_eq!(effects.effects.len(), 0);
    }
}
