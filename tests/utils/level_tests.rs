#[cfg(test)]
mod tests {
    use badge_forge::utils::level::calculate_level;

    #[test]
    fn test_basic_calculation() {
        // Test basic addition functionality
        assert_eq!(calculate_level(5, 10, 0), 15);
        assert_eq!(calculate_level(10, 5, 2), 17);
        assert_eq!(calculate_level(1, 1, 1), 3);
    }

    #[test]
    fn test_zero_values() {
        // Test cases with zeros
        assert_eq!(calculate_level(0, 0, 0), 0);
        assert_eq!(calculate_level(10, 0, 0), 10);
        assert_eq!(calculate_level(0, 10, 0), 10);
        assert_eq!(calculate_level(0, 0, 10), 10);
    }

    #[test]
    fn test_larger_values() {
        // Test with larger numbers
        assert_eq!(calculate_level(100, 150, 50), 300);
        assert_eq!(calculate_level(250, 250, 100), 600);
        assert_eq!(calculate_level(1000, 500, 200), 1700);
    }

    #[test]
    fn test_level_thresholds() {
        // Test specific level thresholds that might trigger badges
        assert_eq!(calculate_level(40, 40, 20), 100); // Level 100
        assert_eq!(calculate_level(100, 100, 50), 250); // Level 250
        assert_eq!(calculate_level(200, 200, 100), 500); // Level 500
    }

    #[test]
    fn test_asymmetric_contributions() {
        // Test cases where one value dominates
        assert_eq!(calculate_level(100, 1, 0), 101);
        assert_eq!(calculate_level(1, 100, 0), 101);
        assert_eq!(calculate_level(0, 0, 100), 100);
        assert_eq!(calculate_level(500, 0, 0), 500);
        assert_eq!(calculate_level(0, 500, 0), 500);
        assert_eq!(calculate_level(0, 0, 500), 500);
    }

    #[test]
    fn test_upper_bounds() {
        // Test the maximum possible values to ensure no overflow
        let max_u32_third = u32::MAX / 3;

        // These should work without overflowing
        assert_eq!(
            calculate_level(max_u32_third, max_u32_third, max_u32_third),
            max_u32_third * 3
        );
        assert_eq!(calculate_level(0, u32::MAX, 0), u32::MAX);
        assert_eq!(calculate_level(u32::MAX, 0, 0), u32::MAX);
        assert_eq!(calculate_level(0, 0, u32::MAX), u32::MAX);
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn test_overflow() {
        // This should cause an overflow in debug mode
        calculate_level(u32::MAX, 1, 0);
    }

    #[test]
    fn test_level_progression() {
        // Test progression of levels as user adds recipes, gets likes, and receives remixes
        let mut recipes = 5;
        let mut likes = 10;
        let mut remixes = 2;
        let initial_level = calculate_level(recipes, likes, remixes);

        // Add some recipes
        recipes += 5;
        assert_eq!(calculate_level(recipes, likes, remixes), initial_level + 5);

        // Add some likes
        likes += 10;
        assert_eq!(calculate_level(recipes, likes, remixes), initial_level + 15);

        // Add some community remixes of user's recipes
        remixes += 3;
        assert_eq!(calculate_level(recipes, likes, remixes), initial_level + 18);

        // Add all three
        recipes += 10;
        likes += 20;
        remixes += 5;
        assert_eq!(calculate_level(recipes, likes, remixes), initial_level + 53);
    }

    #[test]
    fn test_remix_reward_rules() {
        // Remixer publishes a variation: earns creation XP (+1 recipe)
        let initial_remixer_level = calculate_level(2, 5, 0);
        let remixed_recipes = 2 + 1;
        let new_remixer_level = calculate_level(remixed_recipes, 5, 0);
        assert_eq!(new_remixer_level, initial_remixer_level + 1);

        // Original author receives a community remix: earns author recognition karma (+1 remix)
        let initial_author_level = calculate_level(10, 20, 1);
        let updated_author_level = calculate_level(10, 20, 1 + 1);
        assert_eq!(updated_author_level, initial_author_level + 1);

        // Self-remix: cook forks their own recipe.
        // The cook gets +1 recipe creation XP, but remixes received does NOT increase.
        let initial_self_remix_level = calculate_level(4, 8, 2);
        let self_remixed_recipes = 4 + 1; // new recipe created
        let self_remixes_received = 2; // self-remix does NOT increment received remixes
        let post_self_remix_level = calculate_level(self_remixed_recipes, 8, self_remixes_received);
        assert_eq!(post_self_remix_level, initial_self_remix_level + 1);
    }
}
