# lembas-backend

Provides the API for the lembas recipe service.
This project uses `cargo`. Use `cargo r` to run the server binary.

## Running with Docker

For development, a Postgres instance and admin UI is provided by the default profile:

- `docker compose up -d`.

The API service itself is available on the `prod` profile:

- `docker compose --profile prod up -d --build`

## Development

- Setting ENV to 'dev' in .env runs the server in dev mode. This will clear the database on startup and insert some sample data.
- If `cargo-watch` is installed, the server can be automatically recompiled and run with `cargo watch -x "r -r"`

## Endpoints

Examples for each endpoint are available in [./bruno](./bruno/), and can be read using the Bruno API client. Dates are in ISO format, i.e. `YYYY-MM-DD`.

|Endpoint|Actions
|-|-
|`/`                      |`GET` Returns a simple health check
|`/accounts`              |`GET` Returns all user accounts
|`/accounts/:id`          |`GET` Returns a single user account
|`/days`                  |`GET` Returns a users scheduled recipes
|                         |`POST` Creates a scheduled recipe
|`/days/:id`              |`DELETE` Deletes a scheduled recipe
|`/ingredients`           |`GET` Returns all ingredients plus custom ingredients. Also allows
|                         |`POST` Creates a custom ingredient for the current user
|`/ingredients/:id`       |`GET` Returns a single ingredient
|                         |`POSt` Updates a custom ingredient
|                         |`DELETE` Deletes a custom ingredient
|`/ingredients&query=Foo` |`GET` Searches for ingredients with names like 'Foo'
|`/recipes`               |`GET` Returns all recipes for the current user
|                         |`POST` Creates a recipe for the current user
|`/recipes/:id`           |`GET` Returns all recipes for the current user
|                         |`POST` Updates a recipe.
|                         |`DELETE` Deletes a recipe.
|`/reminders`             |`GET` Returns all ingredient reminders for the current user
|                         |`POST` Creates an ingredient reminder for the current user
|`/reminders/:id`         |`GET` Returns a single reminder
|`/list/from=&to=`        |`GET` Returns a shopping list for the user's planned meals from `from` to `to`

> Note: In general, deeply nested entities are not loaded, and clients should access such entities directly. For example, the `recipe` entity contained in a `day` will not contain ingredients - to access these, query `/recipes` with the ID.

## API Checklist

- Recipes
  - Fetch one ✓
  - Fetch all ✓
  - Create ✓
  - Update ✓
  - Delete ✓
- Ingredients
  - Fetch one ✓
  - Fetch all ✓
  - Search ✓
- Custom Ingredients
  - Fetch all
  - Create ✓
  - Update ✓
  - Delete ✓
- Reminders
  - Fetch one ✓
  - Fetch all ✓
  - Create ✓
  - Update
  - Delete
- Plan Recipes
  - Fetch range ✓
  - Create ✓
  - Delete ✓
- Shopping List
  - Fetch range ✓
- Recipe Scraping
