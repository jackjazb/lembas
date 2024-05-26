use sqlx::postgres::{PgHasArrayType, PgTypeInfo};

use super::utils;

/// Represents an ingredient.
#[derive(serde::Serialize, sqlx::FromRow, Debug, Clone)]
pub struct Ingredient {
    pub ingredient_id: i32,
    pub account_id: Option<i32>,
    pub name: String,
    pub unit: Option<String>,
    pub purchase_unit: f64,
    pub life: i32,
    pub quantity: f64,
}

/// Allows the creation of user ingredients.
#[derive(serde::Deserialize, Debug)]
pub struct IngredientInput {
    name: String,
    unit: String,
    purchase_unit: f64,
    life: i32,
}

// Represents a list of ingredients used on a day.
pub type DayIngredients = (chrono::NaiveDate, Vec<Ingredient>);

/// These must be manually implemented to work around the lack of support for Option.
impl ::sqlx::Type<::sqlx::Postgres> for Ingredient {
    fn type_info() -> ::sqlx::postgres::PgTypeInfo {
        ::sqlx::postgres::PgTypeInfo::with_name("Ingredient")
    }
}
impl<'r> ::sqlx::decode::Decode<'r, ::sqlx::Postgres> for Ingredient
where
    // ingredient_id
    i32: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
    i32: ::sqlx::types::Type<::sqlx::Postgres>,
    // account_id
    Option<i32>: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
    Option<i32>: ::sqlx::types::Type<::sqlx::Postgres>,
    // name
    String: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
    String: ::sqlx::types::Type<::sqlx::Postgres>,
    // unit
    Option<String>: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
    Option<String>: ::sqlx::types::Type<::sqlx::Postgres>,
    // purchase_unit
    f64: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
    f64: ::sqlx::types::Type<::sqlx::Postgres>,
    // life
    i32: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
    i32: ::sqlx::types::Type<::sqlx::Postgres>,
    // quantity
    Option<f64>: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
    Option<f64>: ::sqlx::types::Type<::sqlx::Postgres>,
{
    fn decode(
        value: ::sqlx::postgres::PgValueRef<'r>,
    ) -> ::std::result::Result<
        Self,
        ::std::boxed::Box<
            dyn ::std::error::Error + 'static + ::std::marker::Send + ::std::marker::Sync,
        >,
    > {
        let mut decoder = ::sqlx::postgres::types::PgRecordDecoder::new(value)?;
        let ingredient_id = decoder.try_decode::<i32>()?;

        // This line is distinct from the macro's output.
        // Instead of automatically trying to decode Option, we assume a failed i32 decode is NULL.
        let user_id = decoder.try_decode::<i32>().ok();
        let name = decoder.try_decode::<String>()?;
        let unit = decoder.try_decode::<String>().ok();
        let purchase_unit = decoder.try_decode::<f64>()?;
        let life = decoder.try_decode::<i32>()?;
        let quantity = decoder.try_decode::<f64>()?;
        ::std::result::Result::Ok(Ingredient {
            ingredient_id,
            account_id: user_id,
            name,
            unit,
            purchase_unit,
            life,
            quantity,
        })
    }
}

/// Allows Vec<Ingredient> to be decoded from an ARRAY_AGG() of its members.
impl PgHasArrayType for Ingredient {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        PgTypeInfo::with_name("RECORD[]")
    }
    fn array_compatible(_ty: &PgTypeInfo) -> bool {
        true
    }
}

impl Ingredient {
    /// Returns a clone of the ingredient with quantity set to 0.
    pub fn zeroed_clone(self) -> Self {
        let mut clone = self.clone();
        clone.quantity = 0.0;
        clone
    }

    /// Calculate the number of purchase units required to fulfil the desired quantity.
    pub fn calculate_units_required(&self) -> i32 {
        if self.quantity <= 0.0 || self.purchase_unit <= 0.0 {
            return 0;
        }
        return (self.quantity / self.purchase_unit) as i32;
    }
    /// Fetches a single ingredient by ID.
    pub async fn find_one(
        pool: &sqlx::PgPool,
        account_id: i32,
        ingredient_id: i32,
    ) -> Result<Ingredient, sqlx::Error> {
        sqlx::query_as(
            r#"
			SELECT 
				id as ingredient_id, 
				account_id,
				name, 
				unit, 
				purchase_unit,
				life,
				purchase_unit as quantity
			FROM ingredient
			WHERE account_id = $1 OR account_id IS NULL AND id = $2
			"#,
        )
        .bind(account_id)
        .bind(ingredient_id)
        .fetch_one(pool)
        .await
    }

    /// Fetches all ingredients
    pub async fn find_all(
        pool: &sqlx::PgPool,
        account_id: i32,
    ) -> Result<Vec<Ingredient>, sqlx::Error> {
        sqlx::query_as(
            r#"
			SELECT 
				id as ingredient_id, 
				account_id,
				name, 
				unit, 
				purchase_unit,
				life,
				purchase_unit as quantity
			FROM ingredient
			WHERE account_id = $1 OR account_id IS NULL
			"#,
        )
        .bind(account_id)
        .fetch_all(pool)
        .await
    }

    /// Searches over ingredient names.
    pub async fn search(
        pool: &sqlx::PgPool,
        account_id: i32,
        text: String,
    ) -> Result<Vec<Ingredient>, sqlx::Error> {
        let pattern = format!("%{}%", text);

        sqlx::query_as(
            r#"
			SELECT 
				id as ingredient_id, 
				account_id,
				name, 
				unit, 
				purchase_unit,
				life,
				purchase_unit as quantity
			FROM ingredient
			WHERE account_id = $1 OR account_id IS NULL AND LOWER(name) LIKE LOWER($2)
		"#,
        )
        .bind(account_id)
        .bind(pattern)
        .fetch_all(pool)
        .await
    }

    /// Creates a new ingredient.
    pub async fn create(
        pool: &sqlx::PgPool,
        account_id: i32,
        input: IngredientInput,
    ) -> Result<Ingredient, sqlx::Error> {
        let (id,): (i32,) = sqlx::query_as(
            r#"
			INSERT INTO ingredient (account_id, name, unit, purchase_unit, life)
			VALUES ($1, $2, $3, $4, $5)
			RETURNING ingredient.id
			"#,
        )
        .bind(account_id)
        .bind(input.name)
        .bind(input.unit)
        .bind(input.purchase_unit)
        .bind(input.life)
        .fetch_one(pool)
        .await?;
        Self::find_one(pool, account_id, id).await
    }

    /// Creates a new ingredient.
    pub async fn update(
        pool: &sqlx::PgPool,
        account_id: i32,
        ingredient_id: i32,
        input: IngredientInput,
    ) -> Result<Ingredient, sqlx::Error> {
        sqlx::query(
            r#"
			UPDATE ingredient SET 
				name = $3, 
				unit = $4, 
				purchase_unit = $5,
				life = $6
			WHERE account_id = $1 AND id = $2
			"#,
        )
        .bind(account_id)
        .bind(ingredient_id)
        .bind(input.name)
        .bind(input.unit)
        .bind(input.purchase_unit)
        .bind(input.life)
        .execute(pool)
        .await?;
        Self::find_one(pool, account_id, ingredient_id).await
    }

    // Deletes a custom ingredient.
    pub async fn delete(
        pool: &sqlx::PgPool,
        account_id: i32,
        ingredient_id: i32,
    ) -> Result<(), sqlx::Error> {
        utils::delete_entity(
            pool,
            utils::UserDeletable::Ingredient,
            account_id,
            ingredient_id,
        )
        .await
    }

    /// Returns a list of dates and the ingredients used on said date.
    pub async fn find_used_in_range(
        pool: &sqlx::PgPool,
        account_id: i32,
        from: chrono::NaiveDate,
        to: chrono::NaiveDate,
    ) -> Result<Vec<DayIngredients>, sqlx::Error> {
        println!("{from:?} {to:?}");
        sqlx::query_as(
            r#"
		SELECT 
			day.date,
			ARRAY_AGG((
				ingredient.id,
				ingredient.account_id,
				ingredient.name, 
				ingredient.unit, 
				ingredient.purchase_unit,
				ingredient.life,
				recipe_ingredient.quantity)) as "ingredients"
		FROM day
		LEFT JOIN recipe ON day.recipe_id = recipe.id
		LEFT JOIN recipe_ingredient ON recipe_ingredient.recipe_id = recipe.id
		LEFT JOIN ingredient ON recipe_ingredient.ingredient_id = ingredient.id
		WHERE day.date BETWEEN $1 AND $2 AND recipe.account_id = $3
		GROUP BY day.date
		"#,
        )
        .bind(from)
        .bind(to)
        .bind(account_id)
        .fetch_all(pool)
        .await
    }
}
