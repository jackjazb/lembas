INSERT INTO "user" ("email") VALUES 
	('jackjazb@gmail.com');

INSERT INTO "unit" ("unit") VALUES 
	('ml'), ('g');

INSERT INTO "ingredient" ("unit_id", "name") VALUES 
	('1', 'Flour'),
	('2', 'Water');

INSERT INTO "recipe" ("user_id", "title", "portions") VALUES 
	('1', 'Bread', '1');

INSERT INTO "recipe_ingredient" ("recipe_id", "ingredient_id", "quantity") VALUES
	('1', '1', '500'),
	('1', '2', '400');
