# lembas-backend

Provides the API for the lembas recipe service.

## Development

This project uses `cargo`. Use `cargo r` to run the server binary.

## Endpoints

Examples for each endpoint are available in [./bruno](./bruno/), and can be read using the Bruno API client.

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
  - Fetch range
- Recipe Scraping
