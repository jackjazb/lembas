use std::collections::HashMap;

use lembas::api::ingredient::Ingredient;

/// Resolves a map of IDs to ingredients based on the test data loaded from fixtures/ingredient
pub fn test_ingredients() -> HashMap<String, Ingredient> {
    HashMap::from([
        (
            "Carrot".into(),
            Ingredient::new(2000, "Carrot".into(), None, None, 1, 10, 10),
        ),
        (
            "Flour".into(),
            Ingredient::new(2001, "Flour".into(), None, Some("g".into()), 1000, 1000, 50),
        ),
        (
            "Water".into(),
            Ingredient::new(2002, "Water".into(), None, Some("g".into()), 0, 0, 100),
        ),
        (
            "Salt".into(),
            Ingredient::new(2003, "Salt".into(), None, Some("g".into()), 0, 250, 7),
        ),
        (
            "Apple".into(),
            Ingredient::new(2004, "Apple".into(), None, None, 1, 6, 7),
        ),
    ])
}
