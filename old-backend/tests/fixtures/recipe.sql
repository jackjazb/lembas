-- Inserts a recipe that uses 5 carrots, half the minimum purchase quantity.
INSERT INTO "recipe" 
("id", 	"user_id", 	"name",			"portions",	"steps") VALUES
(1, 	1,			'Five Carrots',	1,			'[]'),
(2, 	1,			'Six Carrots',	1,			'[]');

-- Quantities for the above recipes.
INSERT INTO "recipe_ingredient" 
("recipe_id", 	"ingredient_id", 	"quantity") VALUES
(1, 			2000,					5),
(2, 			2000,					6);

ALTER SEQUENCE recipe_id_seq RESTART WITH 3;