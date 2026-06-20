use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Category {
    Week,
    Month,
    Year,
}

impl Category {
    pub fn badge_name(&self) -> &'static str {
        match self {
            Self::Week => "recipe_of_the_week",
            Self::Month => "recipe_of_the_month",
            Self::Year => "recipe_of_the_year",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "week" => Some(Self::Week),
            "month" => Some(Self::Month),
            "year" => Some(Self::Year),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_category_badge_name() {
        assert_eq!(Category::Week.badge_name(), "recipe_of_the_week");
        assert_eq!(Category::Month.badge_name(), "recipe_of_the_month");
        assert_eq!(Category::Year.badge_name(), "recipe_of_the_year");
    }

    #[test]
    fn test_category_parse() {
        assert_eq!(Category::parse("week"), Some(Category::Week));
        assert_eq!(Category::parse("month"), Some(Category::Month));
        assert_eq!(Category::parse("year"), Some(Category::Year));
        assert_eq!(Category::parse("invalid"), None);
        assert_eq!(Category::parse(""), None);
    }
}
