use sqlx::postgres::{PgHasArrayType, PgTypeInfo};

/// Represents an ingredient.
#[derive(serde::Serialize, sqlx::FromRow, Debug)]
pub struct Ingredient {
    ingredient_id: i32,
    account_id: Option<i32>,
    name: String,
    unit: String,
    purchase_unit: f64,
    quantity: Option<f64>,
}

/// Allows the creation of user ingredients.
#[derive(serde::Deserialize, Debug)]
pub struct IngredientInput {
    name: String,
    unit: String,
    purchase_unit: f64,
}

/// These must be manually implemented to work around the lack of support for Option.
impl ::sqlx::Type<::sqlx::Postgres> for Ingredient {
    fn type_info() -> ::sqlx::postgres::PgTypeInfo {
        ::sqlx::postgres::PgTypeInfo::with_name("Ingredient")
    }
}
impl<'r> ::sqlx::decode::Decode<'r, ::sqlx::Postgres> for Ingredient
where
    i32: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
    i32: ::sqlx::types::Type<::sqlx::Postgres>,
    Option<i32>: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
    Option<i32>: ::sqlx::types::Type<::sqlx::Postgres>,
    String: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
    String: ::sqlx::types::Type<::sqlx::Postgres>,
    String: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
    String: ::sqlx::types::Type<::sqlx::Postgres>,
    f64: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
    f64: ::sqlx::types::Type<::sqlx::Postgres>,
    f64: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
    f64: ::sqlx::types::Type<::sqlx::Postgres>,
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
        let unit = decoder.try_decode::<String>()?;
        let purchase_unit = decoder.try_decode::<f64>()?;

        // Same here:
        let quantity = decoder.try_decode::<f64>().ok();
        ::std::result::Result::Ok(Ingredient {
            ingredient_id,
            account_id: user_id,
            name,
            unit,
            purchase_unit,
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
    // Fetches a single ingredient by ID.
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

    // Creates a new ingredient.
    pub async fn create(
        pool: &sqlx::PgPool,
        account_id: i32,
        input: IngredientInput,
    ) -> Result<Ingredient, sqlx::Error> {
        let (id,): (i32,) = sqlx::query_as(
            r#"
			INSERT INTO ingredient (account_id, name, unit, purchase_unit)
			VALUES ($1, $2, $3, $4)
			RETURNING ingredient.id
			"#,
        )
        .bind(account_id)
        .bind(input.name)
        .bind(input.unit)
        .bind(input.purchase_unit)
        .fetch_one(pool)
        .await?;
        Self::find_one(pool, account_id, id).await
    }
}
