use std::error::Error;

use super::utils::parse_date;
use serde::{Deserialize, Serialize};

/// Represents an ingredient.
#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub struct Ingredient {
    /// The ingredient's unique ID
    pub id: i32,
    /// Present if the ingredient is assigned to a user.
    pub user_id: Option<i32>,
    /// The name of the ingredient
    pub name: String,
    /// The unit the ingredient's quantity is measured in
    pub unit: Option<String>,
    /// The minimum usable quantity of the ingredient
    pub minimum_quantity: i32,
    /// The minimum buyable quantity of the ingredient
    pub purchase_quantity: i32,
    /// The estimated shelf life of the ingredient
    pub life: i32,
}

impl Ingredient {
    pub fn new(
        id: i32,
        name: String,
        user_id: Option<i32>,
        unit: Option<String>,
        minimum_quantity: i32,
        purchase_quantity: i32,
        life: i32,
    ) -> Self {
        Ingredient {
            id,
            user_id,
            name,
            unit,
            minimum_quantity,
            purchase_quantity,
            life,
        }
    }

    /// Takes an arbitrary amount and scales it to the minimum purchase of this ingredient needed to cover it.
    pub fn scale_purchase_quantity(&self, quantity: i32) -> i32 {
        (quantity as f64 / self.purchase_quantity as f64).ceil() as i32 * self.purchase_quantity
    }
}

/// An input to create a new ingredient.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct IngredientInput {
    name: String,
    unit: Option<String>,
    pub minimum_quantity: i32,
    pub purchase_quantity: i32,
    pub life: i32,
}

/// Creates a new ingredient against a user.
pub async fn create(
    pool: &sqlx::Pool<sqlx::Postgres>,
    user_id: i32,
    ingredient: IngredientInput,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO ingredient (user_id, name, unit, minimum_quantity, purchase_quantity, life) 
		SELECT $1, $2, $3, $4, $5, $6
		RETURNING id",
        user_id,
        ingredient.name,
        ingredient.unit,
        ingredient.minimum_quantity,
        ingredient.purchase_quantity,
        ingredient.life
    )
    .fetch_one(pool)
    .await?;

    Ok(())
}

/// Creates a new ingredient against a user.
pub async fn delete(
    pool: &sqlx::Pool<sqlx::Postgres>,
    user_id: i32,
    ingredient_id: i32,
) -> Result<(), sqlx::Error> {
    // Users shouldn't be able to delete system ingredients.
    sqlx::query!(
        "DELETE FROM ingredient WHERE id=$1 AND user_id=$2",
        ingredient_id,
        user_id,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn query_multiple(
    pool: &sqlx::Pool<sqlx::Postgres>,
    user_id: i32,
) -> Result<Vec<Ingredient>, sqlx::Error> {
    let rows = sqlx::query!(
        "SELECT
		i.id,
		i.name,
        i.user_id,
		i.unit,
		i.minimum_quantity,
		i.purchase_quantity,
		i.life,
		COUNT(ri.ingredient_id)
	  	FROM ingredient as i
	  		LEFT JOIN recipe_ingredient as ri ON i.id = ri.ingredient_id
	  	WHERE i.user_id IS NULL OR i.user_id = $1
	  	GROUP BY i.id
	  	ORDER BY i.user_id DESC, count DESC
		",
        user_id
    )
    .fetch_all(pool)
    .await?;

    let mut ingredients = Vec::new();
    for row in rows {
        let ingredient = Ingredient {
            id: row.id,
            user_id: row.user_id,
            name: row.name,
            unit: row.unit,
            minimum_quantity: row.minimum_quantity,
            purchase_quantity: row.purchase_quantity,
            life: row.life,
        };
        ingredients.push(ingredient);
    }

    Ok(ingredients)
}

pub async fn query_user(
    pool: &sqlx::Pool<sqlx::Postgres>,
    user_id: i32,
) -> Result<Vec<Ingredient>, sqlx::Error> {
    let rows = sqlx::query!(
        "SELECT
		i.id,
		i.name,
        i.user_id,
		i.unit,
		i.minimum_quantity,
		i.purchase_quantity,
		i.life,
		COUNT(ri.ingredient_id)
	  	FROM ingredient as i
	  		LEFT JOIN recipe_ingredient as ri ON i.id = ri.ingredient_id
	  	WHERE i.user_id = $1
	  	GROUP BY i.id
	  	ORDER BY i.user_id DESC, count DESC
		",
        user_id
    )
    .fetch_all(pool)
    .await?;

    let mut ingredients = Vec::new();
    for row in rows {
        let ingredient = Ingredient {
            id: row.id,
            user_id: row.user_id,
            name: row.name,
            unit: row.unit,
            minimum_quantity: row.minimum_quantity,
            purchase_quantity: row.purchase_quantity,
            life: row.life,
        };
        ingredients.push(ingredient);
    }

    Ok(ingredients)
}

/// Represents a recurring purchase of an ingredient derived from a start date
/// and interval in days.
#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub struct ScheduledIngredient {
    pub id: i32,
    pub ingredient: Ingredient,
    pub start_date: String,
    pub interval: i32,
}

pub async fn query_scheduled(
    pool: &sqlx::Pool<sqlx::Postgres>,
    user_id: i32,
) -> Result<Vec<ScheduledIngredient>, sqlx::Error> {
    let rows = sqlx::query!(
        "SELECT
			ins.id,
			ins.start_date, 
			ins.interval, 

			i.id as ingredient_id,
			i.name, 
            i.user_id,
			i.unit,
			i.minimum_quantity, 
			i.purchase_quantity,
			i.life 
		FROM ingredientschedule AS ins
			LEFT JOIN ingredient AS i
			ON i.id = ins.ingredient_id
		WHERE ins.user_id = $1",
        user_id
    )
    .fetch_all(pool)
    .await?;

    let mut items: Vec<ScheduledIngredient> = vec![];

    for row in rows {
        let ingredient = Ingredient::new(
            row.ingredient_id,
            row.name,
            row.user_id,
            row.unit,
            row.minimum_quantity,
            row.purchase_quantity,
            row.life,
        );
        items.push(ScheduledIngredient {
            id: row.id,
            ingredient,
            start_date: row.start_date.to_string(),
            interval: row.interval,
        });
    }
    Ok(items)
}
/// Represents an API input to create a recurring purchase of an ingredient derived from a start date
/// and interval in days.
#[derive(Debug, serde::Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct ScheduledIngredientInput {
    ingredient_id: i32,
    pub start_date: String,
    pub interval: i32,
}

/// Create a new scheduled ingredient purchase for the current user.
pub async fn create_scheduled(
    pool: &sqlx::Pool<sqlx::Postgres>,
    user_id: i32,
    scheduled_ingredient: ScheduledIngredientInput,
) -> Result<i32, Box<dyn Error>> {
    struct Id {
        id: i32,
    }
    let ingredient = sqlx::query_as!(
        Id,
        "INSERT INTO ingredientschedule (user_id, ingredient_id, start_date, interval) 
		SELECT $1, $2, $3, $4
		RETURNING id",
        user_id,
        scheduled_ingredient.ingredient_id,
        parse_date(&scheduled_ingredient.start_date)?,
        scheduled_ingredient.interval
    )
    .fetch_one(pool)
    .await?;

    Ok(ingredient.id)
}

/// Create a new scheduled ingredient purchase for the current user.
pub async fn delete_scheduled(
    pool: &sqlx::Pool<sqlx::Postgres>,
    user_id: i32,
    schedule_id: i32,
) -> Result<(), Box<dyn Error>> {
    sqlx::query!(
        "DELETE FROM ingredientschedule WHERE user_id = $1 AND id = $2",
        user_id,
        schedule_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Represents an ingredient in the context of a recipe
/// - `ingredient` - The ingredient the quantity is relevant to
/// - `quantity` - The quantity of the ingredient
#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub struct IngredientQuantity {
    pub ingredient: Ingredient,
    pub quantity: i32,
}

impl IngredientQuantity {
    pub fn new(ingredient: Ingredient, quantity: i32) -> Self {
        IngredientQuantity {
            ingredient,
            quantity,
        }
    }
}

pub async fn search(
    pool: &sqlx::Pool<sqlx::Postgres>,
    user_id: i32,
    query: String,
) -> Result<Vec<Ingredient>, sqlx::Error> {
    let escaped = format!("%{}%", query);
    sqlx::query_as!(
        Ingredient,
        "SELECT 
            id, 
            user_id, 
            name, 
            unit, 
            minimum_quantity, 
            purchase_quantity, 
            life 
        FROM 
            ingredient 
        WHERE 
            LOWER(name) LIKE LOWER($1) AND (user_id IS NULL OR user_id = $2)
        ORDER BY
            user_id ASC
            ",
        escaped,
        user_id
    )
    .fetch_all(pool)
    .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ingredient_purchase_scaling() {
        let ingredient = Ingredient::new(1, "Test Ingredient".into(), None, None, 50, 50, 7);
        let scaled_purchase = ingredient.scale_purchase_quantity(30);
        assert_eq!(scaled_purchase, 50);
    }

    #[test]
    fn test_ingredient_purchase_scaling_large() {
        let ingredient = Ingredient::new(1, "Test Ingredient".into(), None, None, 50, 50, 7);
        let scaled_purchase = ingredient.scale_purchase_quantity(140);
        assert_eq!(scaled_purchase, 150);
    }
}
