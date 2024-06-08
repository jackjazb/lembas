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
		
		INSERT INTO "ingredient" ("id", "account_id", "unit", "name", "purchase_unit", "life") VALUES 
			('1', '1', 'g', 'Flour', '1500', '365'),
			('2', NULL, 'ml', 'Water', '0', '365'),
			('3', NULL, 'g', 'Tomato Sauce', '750', '7'),
			('4', NULL, 'g', 'Sugar', '500', '7'),
			('5', NULL, 'g', 'Pasta', '500', '365'),
			('6', NULL, NULL, 'Apple', '6', '7');
		SELECT setval('ingredient_id_seq', 6);

		INSERT INTO "reminder" ("id", "account_id",  "ingredient_id", "start", "interval") VALUES
			('1', '1', '1', NOW(), '7');
		SELECT setval('reminder_id_seq', 1);

		INSERT INTO "recipe" ("id", "account_id", "title", "portions", "steps") VALUES 
			('1', '1', 'Bread', '1', '{"Mix dough", "Ferment for 6 hours", "Shape", "Prove for 6 hours", "Bake"}'),
			('2', '1', 'Tomato Pasta', '1', '{"Heat sauce", "Cook pasta", "Combine and serve with basil"}'),
			('3', '1', 'Apple Pie', '12', '{"Chop apples", "Make pastry", "Combine and bake for 1 hour"}');
		SELECT setval('recipe_id_seq', 3);

		INSERT INTO "recipe_ingredient" ("recipe_id", "ingredient_id", "quantity") VALUES
			('1', '1', '500'),
			('1', '2', '400'),
			('2', '3', '100'),
			('2', '5', '75'),
			('3', '1', '400'),
			('3', '6', '12');

		INSERT INTO "day" ("id", "account_id", "date", "recipe_id") VALUES
			('1', '1', '2024-02-01', '1'),
			('2', '1', '2024-04-01', '1'),
			('3', '1', now(), '1');
		SELECT setval('day_id_seq', 3);
		"#,
    )
    .execute(pool)
    .await?;

    Ok(())
}
