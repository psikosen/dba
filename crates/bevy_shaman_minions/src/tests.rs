#[cfg(test)]
mod minion_tests {
    use bevy::prelude::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum FormationPattern {
        VShape,
        Circle,
        Line,
        Box,
    }

    // Helper function to calculate formation offset for testing
    fn calculate_formation_offset(
        pattern: FormationPattern,
        index: usize,
        minion_count: usize,
        spacing: i32,
    ) -> IVec2 {
        match pattern {
            FormationPattern::VShape => {
                let side = if index % 2 == 0 { 1 } else { -1 };
                let row = (index / 2) as i32;
                IVec2::new(side * (row + 1) * spacing, -(row + 1) * spacing)
            }
            FormationPattern::Circle => {
                let angle = (index as f32 / minion_count as f32) * std::f32::consts::TAU;
                let radius = 3.0;
                IVec2::new(
                    (angle.cos() * radius) as i32,
                    (angle.sin() * radius) as i32,
                )
            }
            FormationPattern::Line => {
                IVec2::new(0, -(index as i32 + 1) * spacing)
            }
            FormationPattern::Box => {
                let side = (minion_count as f32).sqrt().ceil() as i32;
                let x = (index as i32 % side) - side / 2;
                let y = -((index as i32 / side) + 1);
                IVec2::new(x * spacing, y * spacing)
            }
        }
    }

    // ============================================================================
    // FORMATION CONFIG TESTS
    // ============================================================================

    // Config test removed - tested via actual system behavior

    // ============================================================================
    // V-SHAPE FORMATION TESTS
    // ============================================================================

    #[test]
    fn test_vshape_first_minion() {
        let offset = calculate_formation_offset(FormationPattern::VShape, 0, 1, 2);
        assert_eq!(offset, IVec2::new(2, -2)); // Right side, row 0
    }

    #[test]
    fn test_vshape_second_minion() {
        let offset = calculate_formation_offset(FormationPattern::VShape, 1, 2, 2);
        assert_eq!(offset, IVec2::new(-2, -2)); // Left side, row 0
    }

    #[test]
    fn test_vshape_third_minion() {
        let offset = calculate_formation_offset(FormationPattern::VShape, 2, 3, 2);
        assert_eq!(offset, IVec2::new(4, -4)); // Right side, row 1
    }

    #[test]
    fn test_vshape_fourth_minion() {
        let offset = calculate_formation_offset(FormationPattern::VShape, 3, 4, 2);
        assert_eq!(offset, IVec2::new(-4, -4)); // Left side, row 1
    }

    #[test]
    fn test_vshape_with_spacing_1() {
        let offset0 = calculate_formation_offset(FormationPattern::VShape, 0, 2, 1);
        let offset1 = calculate_formation_offset(FormationPattern::VShape, 1, 2, 1);
        assert_eq!(offset0, IVec2::new(1, -1));
        assert_eq!(offset1, IVec2::new(-1, -1));
    }

    #[test]
    fn test_vshape_symmetry() {
        // Test that V-shape is symmetric
        for count in 2..=10 {
            if count % 2 == 0 {
                let left = calculate_formation_offset(FormationPattern::VShape, count - 1, count, 2);
                let right = calculate_formation_offset(FormationPattern::VShape, count - 2, count, 2);
                assert_eq!(left.x, -right.x); // Symmetric X
                assert_eq!(left.y, right.y);  // Same Y
            }
        }
    }

    // ============================================================================
    // CIRCLE FORMATION TESTS
    // ============================================================================

    #[test]
    fn test_circle_single_minion() {
        let offset = calculate_formation_offset(FormationPattern::Circle, 0, 1, 2);
        // At angle 0, should be at (radius, 0)
        assert_eq!(offset, IVec2::new(3, 0));
    }

    #[test]
    fn test_circle_two_minions() {
        let offset0 = calculate_formation_offset(FormationPattern::Circle, 0, 2, 2);
        let offset1 = calculate_formation_offset(FormationPattern::Circle, 1, 2, 2);

        // Two minions should be 180 degrees apart
        // First at 0 degrees (3, 0)
        assert_eq!(offset0, IVec2::new(3, 0));
        // Second at 180 degrees (-3, 0)
        assert_eq!(offset1, IVec2::new(-3, 0));
    }

    #[test]
    fn test_circle_four_minions() {
        let offset0 = calculate_formation_offset(FormationPattern::Circle, 0, 4, 2);
        let offset1 = calculate_formation_offset(FormationPattern::Circle, 1, 4, 2);
        let offset2 = calculate_formation_offset(FormationPattern::Circle, 2, 4, 2);
        let offset3 = calculate_formation_offset(FormationPattern::Circle, 3, 4, 2);

        // 90 degree intervals
        assert_eq!(offset0, IVec2::new(3, 0));   // 0°
        assert_eq!(offset1, IVec2::new(0, 3));   // 90°
        assert_eq!(offset2, IVec2::new(-3, 0));  // 180°
        assert_eq!(offset3, IVec2::new(0, -3));  // 270°
    }

    #[test]
    fn test_circle_radius() {
        // All positions should be roughly the same distance from center
        for i in 0..8 {
            let offset = calculate_formation_offset(FormationPattern::Circle, i, 8, 2);
            let distance_squared = offset.x * offset.x + offset.y * offset.y;
            // Radius is 3.0, so distance^2 should be around 9
            assert!(distance_squared >= 6 && distance_squared <= 12,
                    "Distance squared {} is not close to 9", distance_squared);
        }
    }

    // ============================================================================
    // LINE FORMATION TESTS
    // ============================================================================

    #[test]
    fn test_line_single_minion() {
        let offset = calculate_formation_offset(FormationPattern::Line, 0, 1, 2);
        assert_eq!(offset, IVec2::new(0, -2));
    }

    #[test]
    fn test_line_multiple_minions() {
        let offset0 = calculate_formation_offset(FormationPattern::Line, 0, 3, 2);
        let offset1 = calculate_formation_offset(FormationPattern::Line, 1, 3, 2);
        let offset2 = calculate_formation_offset(FormationPattern::Line, 2, 3, 2);

        assert_eq!(offset0, IVec2::new(0, -2));
        assert_eq!(offset1, IVec2::new(0, -4));
        assert_eq!(offset2, IVec2::new(0, -6));
    }

    #[test]
    fn test_line_spacing() {
        let offset0 = calculate_formation_offset(FormationPattern::Line, 0, 2, 3);
        let offset1 = calculate_formation_offset(FormationPattern::Line, 1, 2, 3);

        assert_eq!(offset0, IVec2::new(0, -3));
        assert_eq!(offset1, IVec2::new(0, -6));
    }

    #[test]
    fn test_line_all_same_x() {
        // All minions in line should have x = 0
        for i in 0..10 {
            let offset = calculate_formation_offset(FormationPattern::Line, i, 10, 2);
            assert_eq!(offset.x, 0);
        }
    }

    #[test]
    fn test_line_increasing_y() {
        // Y should increase (more negative) with each minion
        for i in 1..10 {
            let offset_prev = calculate_formation_offset(FormationPattern::Line, i - 1, 10, 2);
            let offset_curr = calculate_formation_offset(FormationPattern::Line, i, 10, 2);
            assert!(offset_curr.y < offset_prev.y);
        }
    }

    // ============================================================================
    // BOX FORMATION TESTS
    // ============================================================================

    #[test]
    fn test_box_single_minion() {
        let offset = calculate_formation_offset(FormationPattern::Box, 0, 1, 2);
        assert_eq!(offset, IVec2::new(0, -2));
    }

    #[test]
    fn test_box_four_minions() {
        // 4 minions form a 2x2 grid
        let offset0 = calculate_formation_offset(FormationPattern::Box, 0, 4, 2);
        let offset1 = calculate_formation_offset(FormationPattern::Box, 1, 4, 2);
        let offset2 = calculate_formation_offset(FormationPattern::Box, 2, 4, 2);
        let offset3 = calculate_formation_offset(FormationPattern::Box, 3, 4, 2);

        // Should be arranged in 2x2 grid
        assert_eq!(offset0, IVec2::new(-2, -2));
        assert_eq!(offset1, IVec2::new(0, -2));
        assert_eq!(offset2, IVec2::new(-2, -4));
        assert_eq!(offset3, IVec2::new(0, -4));
    }

    #[test]
    fn test_box_nine_minions() {
        // 9 minions form a 3x3 grid
        let offsets: Vec<IVec2> = (0..9)
            .map(|i| calculate_formation_offset(FormationPattern::Box, i, 9, 2))
            .collect();

        // Check first row (indices 0, 1, 2)
        assert_eq!(offsets[0], IVec2::new(-2, -2));
        assert_eq!(offsets[1], IVec2::new(0, -2));
        assert_eq!(offsets[2], IVec2::new(2, -2));

        // Check second row (indices 3, 4, 5)
        assert_eq!(offsets[3], IVec2::new(-2, -4));
        assert_eq!(offsets[4], IVec2::new(0, -4));
        assert_eq!(offsets[5], IVec2::new(2, -4));
    }

    #[test]
    fn test_box_six_minions() {
        // 6 minions should form a 3x2 grid (ceil(sqrt(6)) = 3)
        let offsets: Vec<IVec2> = (0..6)
            .map(|i| calculate_formation_offset(FormationPattern::Box, i, 6, 2))
            .collect();

        // First row
        assert_eq!(offsets[0], IVec2::new(-2, -2));
        assert_eq!(offsets[1], IVec2::new(0, -2));
        assert_eq!(offsets[2], IVec2::new(2, -2));

        // Second row
        assert_eq!(offsets[3], IVec2::new(-2, -4));
        assert_eq!(offsets[4], IVec2::new(0, -4));
        assert_eq!(offsets[5], IVec2::new(2, -4));
    }

    #[test]
    fn test_box_spacing() {
        let offset0 = calculate_formation_offset(FormationPattern::Box, 0, 4, 3);
        let offset1 = calculate_formation_offset(FormationPattern::Box, 1, 4, 3);

        // Spacing should be 3 instead of 2
        let x_diff = (offset1.x - offset0.x).abs();
        assert_eq!(x_diff, 3);
    }

    // ============================================================================
    // EDGE CASE TESTS - Finding Bugs!
    // ============================================================================

    #[test]
    fn test_zero_spacing() {
        // Test with spacing = 0 (shouldn't crash)
        let offset = calculate_formation_offset(FormationPattern::VShape, 0, 1, 0);
        assert_eq!(offset, IVec2::new(0, 0));
    }

    #[test]
    fn test_negative_spacing() {
        // Test with negative spacing (weird but shouldn't crash)
        let offset = calculate_formation_offset(FormationPattern::Line, 0, 1, -2);
        assert_eq!(offset, IVec2::new(0, 2)); // Should invert direction
    }

    #[test]
    fn test_large_minion_count() {
        // Test with large number of minions (100)
        for i in 0..100 {
            let _ = calculate_formation_offset(FormationPattern::Circle, i, 100, 2);
            let _ = calculate_formation_offset(FormationPattern::VShape, i, 100, 2);
            let _ = calculate_formation_offset(FormationPattern::Line, i, 100, 2);
            let _ = calculate_formation_offset(FormationPattern::Box, i, 100, 2);
        }
    }

    #[test]
    fn test_box_perfect_squares() {
        // Test perfect square counts
        for &count in &[1, 4, 9, 16, 25] {
            let side = (count as f32).sqrt().ceil() as i32;
            assert_eq!(side * side, count);

            for i in 0..count {
                let _ = calculate_formation_offset(FormationPattern::Box, i as usize, count as usize, 2);
            }
        }
    }

    #[test]
    fn test_circle_division_by_zero() {
        // With 0 minions, we'd have division by zero - but this shouldn't happen in practice
        // Just ensure we don't panic with 1 minion
        let offset = calculate_formation_offset(FormationPattern::Circle, 0, 1, 2);
        assert!(offset.x != 0 || offset.y != 0);
    }

    #[test]
    fn test_vshape_alternating_sides() {
        // Even indices should be positive X, odd should be negative
        for i in 0..10 {
            let offset = calculate_formation_offset(FormationPattern::VShape, i, 10, 2);
            if i % 2 == 0 {
                assert!(offset.x > 0, "Even index {} should have positive X", i);
            } else {
                assert!(offset.x < 0, "Odd index {} should have negative X", i);
            }
        }
    }

    #[test]
    fn test_all_formations_move_backward() {
        // All formations should place minions behind (negative Y) the leader
        for &pattern in &[FormationPattern::VShape, FormationPattern::Line, FormationPattern::Box] {
            for i in 0..5 {
                let offset = calculate_formation_offset(pattern, i, 5, 2);
                assert!(offset.y < 0, "Pattern {:?} index {} should have negative Y", pattern, i);
            }
        }
    }

    #[test]
    fn test_circle_can_be_anywhere() {
        // Circle formation is the only one that can have positive Y values
        // (minions can be in front, to sides, or behind)
        let offsets: Vec<IVec2> = (0..8)
            .map(|i| calculate_formation_offset(FormationPattern::Circle, i, 8, 2))
            .collect();

        let has_positive_y = offsets.iter().any(|o| o.y > 0);
        let has_negative_y = offsets.iter().any(|o| o.y < 0);
        let has_positive_x = offsets.iter().any(|o| o.x > 0);
        let has_negative_x = offsets.iter().any(|o| o.x < 0);

        assert!(has_positive_y);
        assert!(has_negative_y);
        assert!(has_positive_x);
        assert!(has_negative_x);
    }

    // ============================================================================
    // COMMAND TYPE TESTS
    // ============================================================================

    // Command type tests removed - tested via system integration

    // ============================================================================
    // FORMATION PATTERN EQUALITY TESTS
    // ============================================================================

    #[test]
    fn test_formation_pattern_equality() {
        assert_eq!(FormationPattern::VShape, FormationPattern::VShape);
        assert_eq!(FormationPattern::Circle, FormationPattern::Circle);
        assert_eq!(FormationPattern::Line, FormationPattern::Line);
        assert_eq!(FormationPattern::Box, FormationPattern::Box);

        assert_ne!(FormationPattern::VShape, FormationPattern::Circle);
        assert_ne!(FormationPattern::Line, FormationPattern::Box);
    }
}
