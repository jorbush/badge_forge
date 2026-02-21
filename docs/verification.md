# User Verification

## Overview

The verification system recognizes users who have made a substantial contribution to the platform. A verified user is one who has demonstrated consistent engagement by publishing a meaningful volume of recipes.

## How Verification Works

A user becomes verified when they have created **30 or more recipes**. At that point, their `verified` field is set to `true` in the database.

Verification is evaluated automatically every time a badge update is processed for a user. No manual action is required.

## Rules

- Verification is **permanent**: once granted, it is never revoked, even if the recipe count somehow drops below 30.
- The `verified` field on the `User` model is `Option<bool>`. It can be `true`, `false`, or absent (`null`) for older records — absent is treated the same as `false`.

## Data Model

```rust
pub struct User {
    // ...
    pub verified: Option<bool>,
}
```

## Threshold

| Recipes created | Verified |
|-----------------|----------|
| < 30            | `false`  |
| ≥ 30            | `true`   |
