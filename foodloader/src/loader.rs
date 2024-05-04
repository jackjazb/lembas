use std::{
    error::Error,
    fs::{self, File},
    io::Write,
};

use sqlx::{Execute, Postgres, QueryBuilder};

use crate::scraper::{Ingredient, INGREDIENTS_FILE};

const QUERY_FILE: &str = "data/ingredients.sql";

pub async fn load() -> Result<(), Box<dyn Error>> {
    println!("Publishing ingredient data to database...");

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:example@localhost:5432/lembas")
        .await?;

    let data = fs::read_to_string(INGREDIENTS_FILE)?;
    let mut ingredients: Vec<Ingredient> = serde_json::from_str(&data)?;

    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
        "DELETE FROM ingredient WHERE user_id IS NULL;\nINSERT INTO ingredient (id, name, unit, minimum_quantity, purchase_quantity, life) VALUES\n",
    );
    let mut row_count = 0;

    let mut separated = query_builder.separated(",\n");

    for ingredient in &ingredients {
        let name = ingredient.name.replace("'", "''");
        separated.push(format!(
            "({},'{}',{},{},{},{})",
            ingredient.id,
            name,
            ingredient
                .unit
                .clone()
                .map(|s| format!("'{}'", s))
                .unwrap_or("NULL".into()),
            ingredient.minimum_quantity,
            ingredient.purchase_quantity,
            ingredient.life
        ));
        row_count += 1;
    }

    ingredients.sort_by(|a, b| b.id.cmp(&a.id));
    let max_id = ingredients.get(0).unwrap().id + 1;

    let query = query_builder.build();
    let reset_query = format!("ALTER SEQUENCE ingredient_id_seq RESTART WITH {max_id}");

    let formatted_query = format!("{};\n{};", query.sql(), reset_query);

    let mut f_ingredients = File::create(QUERY_FILE)?;
    f_ingredients.write_all(formatted_query.as_bytes())?;

    //query.execute(&pool).await?;
    sqlx::query(&reset_query).execute(&pool).await?;

    println!("Inserted {} rows.", row_count);
    println!("Reset pkey sequence to {max_id}",);

    Ok(())
}
