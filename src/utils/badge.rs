use crate::model::recipe::Recipe;
use chrono::{Datelike, NaiveDate};
use std::collections::HashSet;

pub fn assign_badges(user_badges: &mut Vec<String>, user_level: i32, recipes: Vec<Recipe>) {
    // check for level-based badges
    if user_level >= 100 && !user_badges.contains(&"level_100".to_string()) {
        user_badges.push("level_100".to_string());
        if user_level >= 250 && !user_badges.contains(&"level_250".to_string()) {
            user_badges.push("level_250".to_string());
            if user_level >= 500 && !user_badges.contains(&"level_500".to_string()) {
                user_badges.push("level_500".to_string());
            }
        }
    }
    // check for streak badges
    if !user_badges.contains(&"month_streak".to_string()) && is_month_streak(&recipes) {
        user_badges.push("month_streak".to_string());
    }
    if !user_badges.contains(&"week_streak".to_string()) && is_week_streak(&recipes) {
        user_badges.push("week_streak".to_string());
    }
}

// At least one recipe per day for 7 consecutive days
pub fn is_week_streak(recipes: &[Recipe]) -> bool {
    if recipes.len() < 7 {
        return false;
    }

    let dates_with_recipes: HashSet<NaiveDate> =
        recipes.iter().map(|r| r.created_at.date_naive()).collect();

    let mut all_dates: Vec<NaiveDate> = dates_with_recipes.into_iter().collect();
    all_dates.sort();

    if all_dates.len() < 7 {
        return false;
    }

    // Check all possible windows of 7 consecutive days
    'window: for start_idx in 0..=(all_dates.len() - 7) {
        let start_date = all_dates[start_idx];

        // Check if the next 6 dates form a consecutive sequence with start_date
        for day_offset in 1..7 {
            let expected_date = start_date + chrono::Duration::days(day_offset);
            if expected_date != all_dates[start_idx + day_offset as usize] {
                // This window doesn't have consecutive dates
                continue 'window;
            }
        }

        // If we reach here, we found 7 consecutive days
        return true;
    }

    false
}

// At least one recipe per week for 4 consecutive weeks
pub fn is_month_streak(recipes: &[Recipe]) -> bool {
    if recipes.len() < 4 {
        return false;
    }

    let mut week_set = HashSet::new();
    for recipe in recipes {
        // The critical fix: Use iso_week() for BOTH week and year
        let iso_year = recipe.created_at.date_naive().iso_week().year();
        let iso_week = recipe.created_at.date_naive().iso_week().week();
        week_set.insert((iso_year, iso_week));
    }

    let mut sorted_recipes = recipes.to_vec();
    sorted_recipes.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    let most_recent = sorted_recipes[0].created_at.date_naive();
    let mut current_year = most_recent.iso_week().year();
    let mut current_week = most_recent.iso_week().week();

    // Try to find 4 consecutive weeks
    for _ in 0..4 {
        if !week_set.contains(&(current_year, current_week)) {
            return false;
        }

        // Move to previous week
        if current_week > 1 {
            current_week -= 1;
        } else {
            // At week 1, go to last week of previous ISO year
            current_year -= 1;
            current_week = chrono::NaiveDate::from_ymd_opt(current_year, 12, 28)
                .unwrap()
                .iso_week()
                .week();
        }
    }
    true
}
