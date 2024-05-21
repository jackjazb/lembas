use sqlx::PgPool;

/// Loads a preset
pub async fn load_data(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::raw_sql(
        r#"
		DELETE FROM account;
		DELETE FROM ingredient;
		INSERT INTO "account" ("id", "email") VALUES 
		('1', 'jackjazb@gmail.com');
		SELECT setval('user_id_seq', 1);
		
		INSERT INTO "ingredient" ("id", "unit", "name", "purchase_unit") VALUES 
			('1', 'g', 'Flour', '1500'),
			('2', 'ml', 'Water', '0'),
			('3', 'g', 'Sugar', '500');
		SELECT setval('ingredient_id_seq', 3);

		INSERT INTO "reminder" ("id", "account_id",  "ingredient_id", "start", "interval") VALUES
			('1', '1', '1', NOW(), '7');
		SELECT setval('reminder_id_seq', 1);

		INSERT INTO "recipe" ("id", "account_id", "title", "portions", "steps") VALUES 
			('1', '1', 'Bread', '1', '{"Mix dough", "Ferment for 6 hours", "Shape", "Prove for 6 hours", "Bake"}');
		SELECT setval('recipe_id_seq', 1);

		INSERT INTO "recipe_ingredient" ("recipe_id", "ingredient_id", "quantity") VALUES
			('1', '1', '500'),
			('1', '2', '400');
		"#,
    )
    .execute(pool)
    .await?;

    Ok(())
}
