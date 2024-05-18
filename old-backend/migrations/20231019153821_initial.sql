--
-- Create Recipe table
--
DROP TABLE IF EXISTS "recipe" CASCADE;
DROP SEQUENCE IF EXISTS recipe_id_seq;
CREATE SEQUENCE recipe_id_seq INCREMENT 1 MINVALUE 1 MAXVALUE 2147483647 START 2 CACHE 1;

CREATE TABLE "public"."recipe" (
    "id" integer DEFAULT nextval('recipe_id_seq') NOT NULL,
    "user_id" integer NOT NULL,
    "name" character varying NOT NULL,
    "portions" integer NOT NULL,
    "steps" json NOT NULL,
	"created_at" timestamp NOT NULL DEFAULT NOW(),
    CONSTRAINT "recipe_pkey" PRIMARY KEY ("id")
) WITH (oids = false);

--
-- Create Ingredient table
--
DROP TABLE IF EXISTS "ingredient" CASCADE;
DROP SEQUENCE IF EXISTS ingredient_id_seq;
CREATE SEQUENCE ingredient_id_seq INCREMENT 1 MINVALUE 1 MAXVALUE 2147483647 CACHE 1;

CREATE TABLE "public"."ingredient" (
    "id" integer DEFAULT nextval('ingredient_id_seq') NOT NULL,
    "user_id" integer,
    "name" character varying NOT NULL,
    "unit" character varying,
    "minimum_quantity" integer NOT NULL,
    "purchase_quantity" integer NOT NULL,
    "life" integer NOT NULL,
    CONSTRAINT "ingredient_pkey" PRIMARY KEY ("id")
) WITH (oids = false);

COMMENT ON COLUMN "public"."ingredient"."user_id" IS 'An optional relation to a user. All other ingredients are global.';
COMMENT ON COLUMN "public"."ingredient"."minimum_quantity" IS 'The minimum usable quantity.';
COMMENT ON COLUMN "public"."ingredient"."purchase_quantity" IS 'The minimum buyable quantity.';
COMMENT ON COLUMN "public"."ingredient"."life" IS 'The number of days usable after first use.';

--
-- Recipe/Ingredient relation table
--
DROP TABLE IF EXISTS "recipe_ingredient" CASCADE;
CREATE TABLE "public"."recipe_ingredient" (
    "recipe_id" integer NOT NULL,
    "ingredient_id" integer NOT NULL,
    "quantity" integer NOT NULL
) WITH (oids = false);

COMMENT ON COLUMN "public"."recipe_ingredient"."quantity" IS 'Quantity in the ingredient''s unit.';

-- 
-- Possible ingredient substitution table
--
DROP TABLE IF EXISTS "ingredientsubstitution" CASCADE;
DROP SEQUENCE IF EXISTS ingredientsubstitution_id_seq;
CREATE SEQUENCE ingredientsubstitution_id_seq INCREMENT 1 MINVALUE 1 MAXVALUE 2147483647 CACHE 1;

CREATE TABLE "public"."ingredientsubstitution" (
    "id" integer DEFAULT nextval('ingredientsubstitution_id_seq') NOT NULL,
    "user_id" integer NOT NULL,
    "ingredient_id" integer NOT NULL,
    "sub_ingredient_id" integer NOT NULL,
    CONSTRAINT "ingredientsubstitution_pkey" PRIMARY KEY ("id")
) WITH (oids = false);

--
-- Create Day table
--
DROP TABLE IF EXISTS "day" CASCADE;
DROP SEQUENCE IF EXISTS day_id_seq;
CREATE SEQUENCE day_id_seq INCREMENT 1 MINVALUE 1 MAXVALUE 2147483647 CACHE 1;

CREATE TABLE "public"."day" (
    "id" integer DEFAULT nextval('day_id_seq') NOT NULL,
    "recipe_id" integer NOT NULL,
    "date" date NOT NULL,
    CONSTRAINT "day_pkey" PRIMARY KEY ("id")
) WITH (oids = false);

--
-- Create Product Schedule table
--
DROP TABLE IF EXISTS "ingredientschedule" CASCADE;
DROP SEQUENCE IF EXISTS ingredientschedule_id_seq;
CREATE SEQUENCE ingredientschedule_id_seq INCREMENT 1 MINVALUE 1 MAXVALUE 2147483647 CACHE 1;

CREATE TABLE "public"."ingredientschedule" (
    "id" integer DEFAULT nextval('ingredientschedule_id_seq') NOT NULL,
    "user_id" integer NOT NULL,
    "ingredient_id" integer NOT NULL,
    "start_date" date NOT NULL,
    "interval" integer NOT NULL,
    CONSTRAINT "ingredientschedule_pkey" PRIMARY KEY ("id")
) WITH (oids = false);

COMMENT ON COLUMN "public"."ingredientschedule"."interval" IS 'Purchase interval in days.';

--
-- Create a User Account table for development
--
DROP TABLE IF EXISTS "useraccount" CASCADE;
DROP SEQUENCE IF EXISTS user_id_seq;
CREATE SEQUENCE user_id_seq INCREMENT 1 MINVALUE 1 MAXVALUE 2147483647 START 1 CACHE 1;

CREATE TABLE "public"."useraccount" (
    "id" integer DEFAULT nextval('user_id_seq') NOT NULL,
    "idp_id" character varying UNIQUE,
    CONSTRAINT "user_pkey" PRIMARY KEY ("id")
) WITH (oids = false);

--
-- Foreign key constraints for relations
--
ALTER TABLE ONLY "public"."day" ADD CONSTRAINT "day_recipe_id_fkey" FOREIGN KEY (recipe_id) REFERENCES recipe(id) ON UPDATE CASCADE ON DELETE CASCADE NOT DEFERRABLE;

ALTER TABLE ONLY "public"."ingredient" ADD CONSTRAINT "ingredient_user_id_fkey" FOREIGN KEY (user_id) REFERENCES useraccount(id) ON UPDATE CASCADE ON DELETE CASCADE NOT DEFERRABLE;

ALTER TABLE ONLY "public"."ingredientschedule" ADD CONSTRAINT "ingredientschedule_ingredient_id_fkey" FOREIGN KEY (ingredient_id) REFERENCES ingredient(id) ON UPDATE CASCADE ON DELETE CASCADE NOT DEFERRABLE;
ALTER TABLE ONLY "public"."ingredientschedule" ADD CONSTRAINT "ingredientschedule_user_id_fkey" FOREIGN KEY (user_id) REFERENCES useraccount(id) ON UPDATE CASCADE ON DELETE CASCADE NOT DEFERRABLE;

ALTER TABLE ONLY "public"."ingredientsubstitution" ADD CONSTRAINT "ingredientsubstitution_ingredient_id_fkey" FOREIGN KEY (ingredient_id) REFERENCES ingredient(id) ON UPDATE CASCADE ON DELETE CASCADE NOT DEFERRABLE;
ALTER TABLE ONLY "public"."ingredientsubstitution" ADD CONSTRAINT "ingredientsubstitution_sub_ingredient_id_fkey" FOREIGN KEY (sub_ingredient_id) REFERENCES ingredient(id) ON UPDATE CASCADE ON DELETE CASCADE NOT DEFERRABLE;

ALTER TABLE ONLY "public"."recipe" ADD CONSTRAINT "recipe_user_id_fkey" FOREIGN KEY (user_id) REFERENCES useraccount(id) ON UPDATE CASCADE ON DELETE CASCADE NOT DEFERRABLE;

ALTER TABLE ONLY "public"."recipe_ingredient" ADD CONSTRAINT "recipe_ingredient_ingredient_id_fkey" FOREIGN KEY (ingredient_id) REFERENCES ingredient(id) ON UPDATE CASCADE ON DELETE CASCADE NOT DEFERRABLE;
ALTER TABLE ONLY "public"."recipe_ingredient" ADD CONSTRAINT "recipe_ingredient_recipe_id_fkey" FOREIGN KEY (recipe_id) REFERENCES recipe(id) ON UPDATE CASCADE ON DELETE CASCADE NOT DEFERRABLE;