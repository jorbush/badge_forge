#[cfg(test)]
mod level_tests {
    use badge_forge::utils::level::calculate_level;

    #[test]
    fn test_basic_calculation() {
        // Test basic addition functionality
        assert_eq!(calculate_level(5, 10), 15);
        assert_eq!(calculate_level(10, 5), 15);
        assert_eq!(calculate_level(1, 1), 2);
    }

    #[test]
    fn test_zero_values() {
        // Test cases with zeros
        assert_eq!(calculate_level(0, 0), 0);
        assert_eq!(calculate_level(10, 0), 10);
        assert_eq!(calculate_level(0, 10), 10);
    }

    #[test]
    fn test_larger_values() {
        // Test with larger numbers
        assert_eq!(calculate_level(100, 150), 250);
        assert_eq!(calculate_level(250, 250), 500);
        assert_eq!(calculate_level(1000, 500), 1500);
    }

    #[test]
    fn test_level_thresholds() {
        // Test specific level thresholds that might trigger badges
        assert_eq!(calculate_level(50, 50), 100); // Level 100
        assert_eq!(calculate_level(150, 100), 250); // Level 250
        assert_eq!(calculate_level(200, 300), 500); // Level 500
    }

    #[test]
    fn test_asymmetric_contributions() {
        // Test cases where one value dominates
        assert_eq!(calculate_level(100, 1), 101);
        assert_eq!(calculate_level(1, 100), 101);
        assert_eq!(calculate_level(500, 0), 500);
        assert_eq!(calculate_level(0, 500), 500);
    }

    #[test]
    fn test_upper_bounds() {
        // Test the maximum possible values to ensure no overflow
        let max_u32_half = u32::MAX / 2;

        // These should work without overflowing
        assert_eq!(calculate_level(max_u32_half, max_u32_half), u32::MAX - 1);
        assert_eq!(
            calculate_level(max_u32_half - 1, max_u32_half),
            u32::MAX - 2
        );
        assert_eq!(calculate_level(0, u32::MAX), u32::MAX);
        assert_eq!(calculate_level(u32::MAX, 0), u32::MAX);
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn test_overflow() {
        // This should cause an overflow in debug mode
        calculate_level(u32::MAX, 1);
    }

    #[test]
    fn test_level_progression() {
        // Test progression of levels as user adds recipes and gets likes
        let mut recipes = 5;
        let mut likes = 10;
        let initial_level = calculate_level(recipes, likes);

        // Add some recipes
        recipes += 5;
        assert_eq!(calculate_level(recipes, likes), initial_level + 5);

        // Add some likes
        likes += 10;
        assert_eq!(calculate_level(recipes, likes), initial_level + 15);

        // Add both
        recipes += 10;
        likes += 20;
        assert_eq!(calculate_level(recipes, likes), initial_level + 45);
    }
}
