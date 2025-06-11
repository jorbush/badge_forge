# Level System

## Overview

The recipe application uses a straightforward level system that rewards users for both content creation and community recognition. This system provides a simple but effective way to track user engagement and progress within the platform.

## How Levels Are Calculated

```rust
pub fn calculate_level(num_recipes: u32, num_likes: u32) -> u32 {
    num_recipes + num_likes
}
```

A user's level is determined by the sum of:
- The total number of recipes they've created
- The total number of likes they've received

This creates a balanced approach where users can progress by either creating more content or by creating high-quality content that receives community recognition.

## Integration with Badge System

The level system integrates with the badge system to award special achievements at milestone levels:

- **Level 100 Badge**: Awarded when a user reaches level 100
- **Level 250 Badge**: Awarded when a user reaches level 250
- **Level 500 Badge**: Awarded when a user reaches level 500

## Examples

### Example 1: New User
- A new user who has created 3 recipes and received 2 likes will be at level 5.

### Example 2: Content Creator
- A user focused on quantity who has created 75 recipes but only received 25 likes will be at level 100.

### Example 3: Quality Focus
- A user focused on quality who has created 20 recipes but received 80 likes will also be at level 100.

### Example 4: Power User
- An established user who has created 150 recipes and received 350 likes will be at level 500.

## Benefits of the Simple Approach

1. **Transparency**: The level calculation is easy for users to understand
2. **Balanced Incentives**: Rewards both quantity and quality
3. **Immediate Feedback**: Level increases as soon as a user creates content or receives likes
4. **Scalability**: No arbitrary caps on progression
