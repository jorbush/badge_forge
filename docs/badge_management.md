# Badge Management

This module provides functionality for assigning and checking achievement badges based on user activity and level.

## Functions

### `assign_badges`

Evaluates a user's level and recipe history to assign appropriate achievement badges.

```rust
pub fn assign_badges(user_badges: &mut Vec<String>, user_level: i32, recipes: Vec<Recipe>)
```

#### Parameters

- `user_badges`: A mutable reference to the user's current list of badges
- `user_level`: The user's current experience level as an integer
- `recipes`: A collection of the user's recipes with creation timestamps

#### Description

This function assigns achievement badges based on two categories:

1. **Level-based badges**: Awarded when users reach specific experience levels
   - `level_100`: Awarded at level 100
   - `level_250`: Awarded at level 250
   - `level_500`: Awarded at level 500

2. **Streak badges**: Awarded for consistent recipe creation
   - `week_streak`: Awarded for posting at least one recipe per day for 7 consecutive days
   - `month_streak`: Awarded for posting at least one recipe per week for 4 consecutive weeks

Badges are only added if they don't already exist in the user's badge collection.

#### Example

```rust
let mut user_badges = vec!["early_adopter".to_string()];
let user_level = 250;
let recipes = vec![/* user's recipe history */];

assign_badges(&mut user_badges, user_level, recipes);
// user_badges may now contain "level_100", "level_250", "week_streak", etc.
```

---

### `is_week_streak`

Determines if the user has achieved a week streak by posting at least one recipe per day for 7 consecutive days.

```rust
pub fn is_week_streak(recipes: &[Recipe]) -> bool
```

#### Parameters

- `recipes`: A slice of the user's recipes with creation timestamps

#### Returns

`true` if the user has posted at least one recipe per day for 7 consecutive days, `false` otherwise.

#### Algorithm

1. Collects all unique dates on which the user posted recipes
2. Sorts these dates chronologically
3. Uses a sliding window approach to search for any sequence of 7 consecutive days
4. For each potential starting date, verifies if the next 6 days exist in the collection

#### Example

```rust
let recipes = vec![
    Recipe { id: "1", created_at: "2025-01-01T12:00:00Z".parse().unwrap(), ... },
    Recipe { id: "2", created_at: "2025-01-02T18:30:00Z".parse().unwrap(), ... },
    // ... recipes for Jan 3, 4, 5, 6, and 7
];

let has_week_streak = is_week_streak(&recipes); // true
```

#### Notes

- Multiple recipes posted on the same day count as a single day toward the streak
- The 7 consecutive days can occur at any point in the user's history
- At least 7 recipes are required to achieve this badge

---

### `is_month_streak`

Determines if the user has achieved a month streak by posting at least one recipe per week for 4 consecutive weeks.

```rust
pub fn is_month_streak(recipes: &[Recipe]) -> bool
```

#### Parameters

- `recipes`: A slice of the user's recipes with creation timestamps

#### Returns

`true` if the user has posted at least one recipe per week for 4 consecutive weeks, `false` otherwise.

#### Algorithm

1. Groups recipes by ISO week and ISO year
   - Important: Uses ISO week-year standards which can differ from calendar year at year boundaries
2. Sorts recipes by date (most recent first)
3. Starting from the most recent recipe, checks backward for 4 consecutive weeks
4. Handles year boundaries correctly when traversing from week 1 to the previous year

#### Example

```rust
let recipes = vec![
    Recipe { id: "1", created_at: "2025-01-28T12:00:00Z".parse().unwrap(), ... }, // Week 5
    Recipe { id: "2", created_at: "2025-01-21T18:30:00Z".parse().unwrap(), ... }, // Week 4
    Recipe { id: "3", created_at: "2025-01-14T09:15:00Z".parse().unwrap(), ... }, // Week 3
    Recipe { id: "4", created_at: "2025-01-07T22:45:00Z".parse().unwrap(), ... }, // Week 2
];

let has_month_streak = is_month_streak(&recipes); // true
```

#### Notes

- Uses ISO-8601 week numbering system (weeks start on Monday)
- Properly handles week calculations across year boundaries
- Multiple recipes posted in the same ISO week count as a single week toward the streak
- At least 4 recipes are required to achieve this badge

---

## ISO Week Special Considerations

The `is_month_streak` function uses the ISO week date system (ISO-8601), where:

- Weeks start on Monday and end on Sunday
- The first week of a year contains the first Thursday of that year
- Some early January dates might belong to the last week of the previous year
- Some late December dates might belong to the first week of the next year

For example:
- December 31, 2024 belongs to ISO week 1 of ISO year 2025
- January 1, 2023 belongs to ISO week 52 of ISO year 2022

This standard ensures consistent week numbering regardless of which day of the week a year starts on.
