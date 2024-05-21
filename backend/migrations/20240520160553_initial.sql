-- Create the ingredients table.
DROP TABLE IF EXISTS "ingredient" CASCADE;

DROP SEQUENCE IF EXISTS ingredient_id_seq;

CREATE SEQUENCE ingredient_id_seq INCREMENT 1 MINVALUE 1 MAXVALUE 2147483647 CACHE 1;

CREATE TABLE "public"."ingredient" (
	"id" integer DEFAULT nextval('ingredient_id_seq') NOT NULL,
	"account_id" integer,
	"unit" text,
	"name" text NOT NULL,
	"purchase_unit" float NOT NULL,
	"life" integer NOT NULL,
	CONSTRAINT "ingredient_pkey" PRIMARY KEY ("id")
) WITH (oids = false);

-- Create the reminders table.
DROP TABLE IF EXISTS "reminder" CASCADE;

DROP SEQUENCE IF EXISTS reminder_id_seq;

CREATE SEQUENCE reminder_id_seq INCREMENT 1 MINVALUE 1 MAXVALUE 2147483647 CACHE 1;

CREATE TABLE "public"."reminder" (
	"id" integer DEFAULT nextval('reminder_id_seq') NOT NULL,
	"ingredient_id" integer NOT NULL,
	"account_id" integer NOT NULL,
	"start" date NOT NULL,
	"interval" integer NOT NULL,
	CONSTRAINT "reminder_pkey" PRIMARY KEY ("id")
) WITH (oids = false);

-- Create the recipes table.
DROP TABLE IF EXISTS "recipe" CASCADE;

DROP SEQUENCE IF EXISTS recipe_id_seq;

CREATE SEQUENCE recipe_id_seq INCREMENT 1 MINVALUE 1 MAXVALUE 2147483647 CACHE 1;

CREATE TABLE "public"."recipe" (
	"id" integer DEFAULT nextval('recipe_id_seq') NOT NULL,
	"account_id" integer NOT NULL,
	"title" text NOT NULL,
	"portions" integer NOT NULL,
	"steps" text [] NOT NULL,
	CONSTRAINT "recipe_pkey" PRIMARY KEY ("id")
) WITH (oids = false);

-- Link table for recipes and ingredients.
DROP TABLE IF EXISTS "recipe_ingredient" CASCADE;

DROP SEQUENCE IF EXISTS recipe_ingredient_id_seq;

CREATE SEQUENCE recipe_ingredient_id_seq INCREMENT 1 MINVALUE 1 MAXVALUE 2147483647 CACHE 1;

CREATE TABLE "public"."recipe_ingredient" (
	"id" integer DEFAULT nextval('recipe_ingredient_id_seq') NOT NULL,
	"recipe_id" integer NOT NULL,
	"ingredient_id" integer NOT NULL,
	"quantity" float NOT NULL,
	CONSTRAINT "recipe_ingredient_pkey" PRIMARY KEY ("id")
) WITH (oids = false);

-- Create the accounts table
DROP TABLE IF EXISTS "account" CASCADE;

DROP SEQUENCE IF EXISTS account_id_seq;

CREATE SEQUENCE account_id_seq INCREMENT 1 MINVALUE 1 MAXVALUE 2147483647 CACHE 1;

CREATE TABLE "public"."account" (
	"id" integer DEFAULT nextval('account_id_seq') NOT NULL,
	"email" text NOT NULL,
	CONSTRAINT "account_pkey" PRIMARY KEY ("id")
) WITH (oids = false);

-- Recipe/Account foreign key.
ALTER TABLE
	ONLY "public"."recipe"
ADD
	CONSTRAINT "fk-recipe-account" FOREIGN KEY (account_id) REFERENCES "account"(id) ON DELETE CASCADE NOT DEFERRABLE;

-- Ingredient/Account foreign key.
ALTER TABLE
	ONLY "public"."ingredient"
ADD
	CONSTRAINT "fk-ingredient-account" FOREIGN KEY (account_id) REFERENCES "account"(id) ON DELETE CASCADE NOT DEFERRABLE;

-- Reminder/Ingredient foreign key.
ALTER TABLE
	ONLY "public"."reminder"
ADD
	CONSTRAINT "fk-reminder-ingredient" FOREIGN KEY (ingredient_id) REFERENCES "ingredient"(id) ON DELETE CASCADE NOT DEFERRABLE;

-- Reminder/Account foreign key.
ALTER TABLE
	ONLY "public"."reminder"
ADD
	CONSTRAINT "fk-reminder-account" FOREIGN KEY (account_id) REFERENCES "account"(id) ON DELETE CASCADE NOT DEFERRABLE;

-- RecipeIngredient/Ingredient foreign key.
ALTER TABLE
	ONLY "public"."recipe_ingredient"
ADD
	CONSTRAINT "fk-recipeingredient-ingredient" FOREIGN KEY (ingredient_id) REFERENCES ingredient(id) ON DELETE RESTRICT NOT DEFERRABLE;

-- RecipeIngredient/Recipe foreign key.
ALTER TABLE
	ONLY "public"."recipe_ingredient"
ADD
	CONSTRAINT "fk-recipeingredient-recipe" FOREIGN KEY (recipe_id) REFERENCES recipe(id) ON DELETE CASCADE NOT DEFERRABLE;