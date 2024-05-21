use sqlx::postgres::{PgHasArrayType, PgTypeInfo};

/// Represents an ingredient.
#[derive(serde::Serialize, sqlx::Type, sqlx::FromRow, Debug)]
pub struct Ingredient {
    ingredient_id: i32,
    name: String,
    unit: String,
    purchase_unit: f64,
    quantity: f64,
}

/// Allows Vec<Ingredient> to be decoded from an ARRAY_AGG() of its
impl PgHasArrayType for Ingredient {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        PgTypeInfo::with_name("RECORD[]")
    }
    fn array_compatible(_ty: &PgTypeInfo) -> bool {
        true
    }
}

impl Ingredient {}
