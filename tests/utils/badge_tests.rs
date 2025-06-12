#[cfg(test)]
mod badge_tests {
    use badge_forge::{
        model::recipe::Recipe,
        utils::badge::{assign_badges, is_month_streak, is_week_streak},
    };
    use chrono::{DateTime, TimeZone, Utc};
    use mongodb::bson::oid::ObjectId;
    use std::str::FromStr;

    // Helper functions
    fn create_recipe(id: ObjectId, user_id: ObjectId, likes: i32, day_offset: i64) -> Recipe {
        // Use a fixed base date (2025-01-01) + day_offset
        let base = Utc.with_ymd_and_hms(2025, 1, 1, 12, 0, 0).unwrap();
        Recipe {
            _id: id,
            user_id: user_id,
            num_likes: likes,
            created_at: base + chrono::Duration::days(day_offset),
        }
    }

    fn create_recipe_with_date(
        id: ObjectId,
        user_id: ObjectId,
        likes: i32,
        date_str: &str,
    ) -> Recipe {
        Recipe {
            _id: id,
            user_id: user_id,
            num_likes: likes,
            created_at: DateTime::from_str(date_str).unwrap_or(Utc::now()),
        }
    }

    #[test]
    fn test_assign_badges_level_based() {
        // Test level 100 badge
        let mut badges = Vec::new();
        let recipes = vec![]; // No recipes needed for level badges
        assign_badges(&mut badges, 100, recipes);
        assert!(badges.contains(&"level_100".to_string()));
        assert!(!badges.contains(&"level_250".to_string()));
        assert!(!badges.contains(&"level_500".to_string()));

        // Test level 250 badge
        let mut badges = Vec::new();
        let recipes = vec![]; // No recipes needed for level badges
        assign_badges(&mut badges, 250, recipes);
        assert!(badges.contains(&"level_100".to_string()));
        assert!(badges.contains(&"level_250".to_string()));
        assert!(!badges.contains(&"level_500".to_string()));

        // Test level 500 badge
        let mut badges = Vec::new();
        let recipes = vec![]; // No recipes needed for level badges
        assign_badges(&mut badges, 500, recipes);
        assert!(badges.contains(&"level_100".to_string()));
        assert!(badges.contains(&"level_250".to_string()));
        assert!(badges.contains(&"level_500".to_string()));

        // Test no level badges for level below 100
        let mut badges = Vec::new();
        let recipes = vec![]; // No recipes needed for level badges
        assign_badges(&mut badges, 99, recipes);
        assert!(badges.is_empty());
    }

    #[test]
    fn test_assign_badges_no_duplicates() {
        // Test that badges aren't added twice
        let mut badges = vec!["level_100".to_string()];
        let recipes = vec![]; // No recipes needed for level badges
        assign_badges(&mut badges, 100, recipes);
        assert_eq!(badges.len(), 1); // Still just one badge
        assert!(badges.contains(&"level_100".to_string()));
    }

    #[test]
    fn test_is_week_streak() {
        let user_id = ObjectId::new();

        // Test successful week streak
        let recipes = vec![
            create_recipe(ObjectId::new(), user_id, 0, 0),
            create_recipe(ObjectId::new(), user_id, 0, -1),
            create_recipe(ObjectId::new(), user_id, 0, -2),
            create_recipe(ObjectId::new(), user_id, 0, -3),
            create_recipe(ObjectId::new(), user_id, 0, -4),
            create_recipe(ObjectId::new(), user_id, 0, -5),
            create_recipe(ObjectId::new(), user_id, 0, -6),
        ];
        assert!(is_week_streak(&recipes));

        // Test failed week streak - not enough recipes
        let recipes = vec![
            create_recipe(ObjectId::new(), user_id, 0, 0),
            create_recipe(ObjectId::new(), user_id, 0, -1),
            create_recipe(ObjectId::new(), user_id, 0, -2),
            create_recipe(ObjectId::new(), user_id, 0, -3),
            create_recipe(ObjectId::new(), user_id, 0, -4),
            create_recipe(ObjectId::new(), user_id, 0, -5),
        ];
        assert!(!is_week_streak(&recipes));

        // Test failed week streak - gap in days
        let recipes = vec![
            create_recipe(ObjectId::new(), user_id, 0, 0),
            create_recipe(ObjectId::new(), user_id, 0, -1),
            create_recipe(ObjectId::new(), user_id, 0, -2),
            create_recipe(ObjectId::new(), user_id, 0, -3),
            create_recipe(ObjectId::new(), user_id, 0, -5), // Missing day 4
            create_recipe(ObjectId::new(), user_id, 0, -6),
            create_recipe(ObjectId::new(), user_id, 0, -7),
        ];
        assert!(!is_week_streak(&recipes));

        // Test successful week streak - multiple recipes per day
        let recipes = vec![
            create_recipe(ObjectId::new(), user_id, 0, 0),
            create_recipe(ObjectId::new(), user_id, 0, 0), // Same day
            create_recipe(ObjectId::new(), user_id, 0, -1),
            create_recipe(ObjectId::new(), user_id, 0, -2),
            create_recipe(ObjectId::new(), user_id, 0, -2), // Same day
            create_recipe(ObjectId::new(), user_id, 0, -3),
            create_recipe(ObjectId::new(), user_id, 0, -4),
            create_recipe(ObjectId::new(), user_id, 0, -5),
            create_recipe(ObjectId::new(), user_id, 0, -6),
        ];
        assert!(is_week_streak(&recipes));
    }

    #[test]
    fn test_is_month_streak() {
        let user_id = ObjectId::new();

        // Test month streak with ISO weeks
        let recipes = vec![
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2025-01-28T12:00:00Z"), // Week 5
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2025-01-21T12:00:00Z"), // Week 4
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2025-01-14T12:00:00Z"), // Week 3
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2025-01-07T12:00:00Z"), // Week 2
        ];
        assert!(is_month_streak(&recipes));

        // Test failed month streak - not enough recipes
        let recipes = vec![
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2025-01-28T12:00:00Z"),
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2025-01-21T12:00:00Z"),
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2025-01-14T12:00:00Z"),
        ];
        assert!(!is_month_streak(&recipes));

        // Test failed month streak - gap in weeks
        let recipes = vec![
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2025-01-28T12:00:00Z"), // Week 5
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2025-01-21T12:00:00Z"), // Week 4
            // Missing Week 3
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2025-01-07T12:00:00Z"), // Week 2
        ];
        assert!(!is_month_streak(&recipes));

        // Test successful month streak - multiple recipes per week
        let recipes = vec![
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2025-01-28T12:00:00Z"), // Week 5
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2025-01-30T12:00:00Z"), // Week 5 again
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2025-01-21T12:00:00Z"), // Week 4
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2025-01-14T12:00:00Z"), // Week 3
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2025-01-07T12:00:00Z"), // Week 2
        ];
        assert!(is_month_streak(&recipes));
    }

    #[test]
    fn test_year_boundary_cases() {
        let user_id = ObjectId::new();

        // Test week streak crossing year boundary
        let recipes = vec![
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2025-01-03T12:00:00Z"), // Friday, Week 1
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2025-01-02T12:00:00Z"), // Thursday, Week 1
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2025-01-01T12:00:00Z"), // Wednesday, Week 1
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2024-12-31T12:00:00Z"), // Tuesday, Week 1
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2024-12-30T12:00:00Z"), // Monday, Week 1
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2024-12-29T12:00:00Z"), // Sunday, Week 52
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2024-12-28T12:00:00Z"), // Saturday, Week 52
        ];
        assert!(is_week_streak(&recipes));

        // Test month streak crossing year boundary
        let recipes = vec![
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2025-01-14T12:00:00Z"), // Week 3, 2025
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2025-01-07T12:00:00Z"), // Week 2, 2025
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2024-12-31T12:00:00Z"), // Week 1, 2025 (year boundary)
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2024-12-24T12:00:00Z"), // Week 52, 2024
        ];
        println!("Recipes: {:?}", recipes);
        println!("Is month streak: {}", is_month_streak(&recipes));
        assert!(is_month_streak(&recipes));
    }

    #[test]
    fn test_edge_cases_for_week_streak() {
        let user_id = ObjectId::new();

        // Test with exactly 7 recipes on consecutive days
        let recipes = vec![
            create_recipe(ObjectId::new(), user_id, 0, 0),
            create_recipe(ObjectId::new(), user_id, 0, -1),
            create_recipe(ObjectId::new(), user_id, 0, -2),
            create_recipe(ObjectId::new(), user_id, 0, -3),
            create_recipe(ObjectId::new(), user_id, 0, -4),
            create_recipe(ObjectId::new(), user_id, 0, -5),
            create_recipe(ObjectId::new(), user_id, 0, -6),
        ];
        assert!(is_week_streak(&recipes));

        // Test with many recipes spread over time, but only recent ones matter
        let mut recipes = vec![];
        // Add older recipes that shouldn't affect the streak
        for i in 20..40 {
            recipes.push(create_recipe(ObjectId::new(), user_id, 0, -i));
        }
        // Add recent streak
        for i in 0..7 {
            recipes.push(create_recipe(ObjectId::new(), user_id, 0, -i));
        }
        assert!(is_week_streak(&recipes));

        // Test with recipes on days 0, 1, 2, 3, 4, 5, 7 (missing day 6)
        let recipes = vec![
            create_recipe(ObjectId::new(), user_id, 0, 0),
            create_recipe(ObjectId::new(), user_id, 0, -1),
            create_recipe(ObjectId::new(), user_id, 0, -2),
            create_recipe(ObjectId::new(), user_id, 0, -3),
            create_recipe(ObjectId::new(), user_id, 0, -4),
            create_recipe(ObjectId::new(), user_id, 0, -5),
            create_recipe(ObjectId::new(), user_id, 0, -7), // Skip day 6
        ];
        assert!(!is_week_streak(&recipes));
    }

    #[test]
    fn test_edge_cases_for_month_streak() {
        let user_id = ObjectId::new();

        // Test with recipes in non-consecutive weeks
        let recipes = vec![
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2025-01-28T12:00:00Z"), // Week 5
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2025-01-14T12:00:00Z"), // Week 3
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2025-01-07T12:00:00Z"), // Week 2
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2024-12-24T12:00:00Z"), // Week 52
        ];
        assert!(!is_month_streak(&recipes));

        // Test with exactly 4 recipes in 4 consecutive weeks
        let recipes = vec![
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2025-01-28T12:00:00Z"), // Week 5
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2025-01-21T12:00:00Z"), // Week 4
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2025-01-14T12:00:00Z"), // Week 3
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2025-01-07T12:00:00Z"), // Week 2
        ];
        assert!(is_month_streak(&recipes));

        // Test with recipes spanning more than 4 weeks but all consecutive
        let recipes = vec![
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2025-02-04T12:00:00Z"), // Week 6
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2025-01-28T12:00:00Z"), // Week 5
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2025-01-21T12:00:00Z"), // Week 4
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2025-01-14T12:00:00Z"), // Week 3
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2025-01-07T12:00:00Z"), // Week 2
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2024-12-31T12:00:00Z"), // Week 1
        ];
        assert!(is_month_streak(&recipes));
    }

    #[test]
    fn test_combined_scenarios() {
        let user_id = ObjectId::new();

        // Test combined level and streak badges
        let mut badges = Vec::new();

        // Create a week streak (7 days)
        let mut recipes = vec![];
        for i in 0..7 {
            recipes.push(create_recipe(ObjectId::new(), ObjectId::new(), 0, -i));
        }

        // Test with level 250 and week streak
        assign_badges(&mut badges, 250, recipes.clone());
        assert!(badges.contains(&"level_100".to_string()));
        assert!(badges.contains(&"level_250".to_string()));
        assert!(badges.contains(&"week_streak".to_string()));
        assert!(!badges.contains(&"level_500".to_string()));
        assert!(!badges.contains(&"month_streak".to_string()));

        // Test with level 500 and both streaks
        badges.clear();
        let mut month_recipes = vec![
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2025-01-28T12:00:00Z"), // Week 5
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2025-01-21T12:00:00Z"), // Week 4
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2025-01-14T12:00:00Z"), // Week 3
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2025-01-07T12:00:00Z"), // Week 2
        ];
        recipes.append(&mut month_recipes);
        println!("Recipes: {:?}", recipes);
        assign_badges(&mut badges, 500, recipes);
        println!("Badges: {:?}", badges);
        assert!(badges.contains(&"level_100".to_string()));
        assert!(badges.contains(&"level_250".to_string()));
        assert!(badges.contains(&"level_500".to_string()));
        assert!(badges.contains(&"week_streak".to_string()));
        assert!(badges.contains(&"month_streak".to_string()));
        assert_eq!(badges.len(), 5); // All 5 badges
    }

    #[test]
    fn test_empty_recipes() {
        // Test with empty recipes array
        assert!(!is_week_streak(&[]));
        assert!(!is_month_streak(&[]));

        let mut badges = Vec::new();
        assign_badges(&mut badges, 0, vec![]);
        assert!(badges.is_empty());
    }

    #[test]
    fn test_realistic_user_patterns() {
        let user_id = ObjectId::new();

        // Test for a user who posts regularly but misses a day
        let mut recipes = Vec::new();
        // Days 0, 1, 2, 4, 5, 6, 7 (missing day 3)
        for i in [0, 1, 2, 4, 5, 6, 7].iter() {
            recipes.push(create_recipe(
                ObjectId::new(),
                ObjectId::new(),
                0,
                -i as i64,
            ));
        }
        assert!(!is_week_streak(&recipes));

        // Test for a user who posts multiple recipes per day but for fewer than 7 days
        let mut recipes = Vec::new();
        // Days 0, 1, 2, 3, 4 (only 5 days)
        for i in 0..5 {
            recipes.push(create_recipe(ObjectId::new(), user_id, 0, -i));
            recipes.push(create_recipe(ObjectId::new(), user_id, 0, -i));
        }
        assert!(!is_week_streak(&recipes));

        // Test for a user with a complex posting pattern
        // Week 1: 3 recipes
        // Week 2: 2 recipes
        // Week 3: 1 recipe
        // Week 4: 2 recipes
        // Week 5: 0 recipes
        // Week 6: 4 recipes
        let recipes = vec![
            // Week 6
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2025-02-07T12:00:00Z"),
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2025-02-06T12:00:00Z"),
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2025-02-05T12:00:00Z"),
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2025-02-04T12:00:00Z"),
            // Week 4
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2025-01-24T12:00:00Z"),
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2025-01-21T12:00:00Z"),
            // Week 3
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2025-01-14T12:00:00Z"),
            // Week 2
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2025-01-09T12:00:00Z"),
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2025-01-07T12:00:00Z"),
            // Week 1
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2025-01-03T12:00:00Z"),
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2025-01-02T12:00:00Z"),
            create_recipe_with_date(ObjectId::new(), user_id, 0, "2025-01-01T12:00:00Z"),
        ];

        // Should not have month streak (gap in Week 5)
        assert!(!is_month_streak(&recipes));

        // Should not have week streak (recipes in Week 6 are not on consecutive days)
        assert!(!is_week_streak(&recipes));
    }
}
