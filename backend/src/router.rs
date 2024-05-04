use std::{collections::HashMap, error::Error, str::FromStr};

use axum::{
    extract::{Path, Query, State, TypedHeader},
    headers::authorization,
    http::{Request, StatusCode},
    middleware, response,
    routing::{delete, get, post, put},
    Extension, Json, Router,
};
use jsonwebtoken::{
    decode, decode_header,
    jwk::{self, JwkSet},
    Algorithm, DecodingKey, Validation,
};
use serde::Deserialize;
use sqlx::query_as;
use tracing::{event, Level};

use crate::api::{
    day::{self, Day, DayInput, ShoppingList},
    db::Db,
    ingredient::{
        self, Ingredient, IngredientInput, ScheduledIngredient, ScheduledIngredientInput,
    },
    recipe::{self, Recipe, RecipeInput},
};

type ServerError = (StatusCode, String);

struct User {
    id: i32,
}

#[derive(Clone)]
struct AuthState {
    db: Db,
    jwks: Option<jwk::JwkSet>,
}

/// Verify a JWT against the current IDP's keystore. Returns the request sub if validation suceeds.
async fn verify_token(jwks: JwkSet, token: &str) -> Result<String, Box<dyn Error>> {
    let header = decode_header(&token)?;

    let kid = header.kid.ok_or("KID missing")?;

    if let Some(j) = jwks.find(&kid) {
        match &j.algorithm {
            jwk::AlgorithmParameters::RSA(rsa) => {
                let decoding_key = DecodingKey::from_rsa_components(&rsa.n, &rsa.e)?;

                let mut validation = Validation::new(
                    Algorithm::from_str(j.common.key_algorithm.unwrap().to_string().as_str())
                        .unwrap(),
                );
                validation.validate_exp = false;
                let decoded_token = decode::<HashMap<String, serde_json::Value>>(
                    token,
                    &decoding_key,
                    &validation,
                )?;

                return Ok(decoded_token
                    .claims
                    .get("sub")
                    .ok_or("Sub field missing.")?
                    .as_str()
                    .ok_or("Couldn't parse sub")?
                    .to_string());
            }
            _ => unreachable!("This should be a RSA"),
        }
    }

    Err("Verification failed.".into())
}

/// Authorises requests to the server by checking the bearer token. If IDP support is enabled,
/// `user_id` is mapped from the `sub` field - otherwise, whatever bearer token was passed is used.
async fn authorise<B>(
    TypedHeader(auth): TypedHeader<authorization::Authorization<authorization::Bearer>>,
    State(auth_state): State<AuthState>,
    mut request: Request<B>,
    next: middleware::Next<B>,
) -> Result<response::Response, StatusCode> {
    let sub = match auth_state.jwks {
        Some(jwks) => {
            let result = verify_token(jwks, auth.token()).await;
            if result.is_err() {
                event!(
                    Level::ERROR,
                    "could not verify auth token '{}'",
                    auth.token()
                )
            }
            result.map_err(|_| StatusCode::UNAUTHORIZED)?
        }
        None => auth.token().to_string(),
    };
    // Fetch user's ID, or make them an account (if Cognito check passes)
    let user_id: Option<i32> =
        match query_as!(User, "SELECT id FROM useraccount WHERE idp_id = $1", sub)
            .fetch_one(&auth_state.db.pool)
            .await
        {
            Ok(user) => Some(user.id),
            Err(_) =>
            // Will need to call Cognito here.
            {
                query_as!(
                    User,
                    "INSERT INTO useraccount (idp_id) VALUES ($1) RETURNING id",
                    sub,
                )
                .fetch_one(&auth_state.db.pool)
                .await
                .map(|user| user.id)
                .ok()
            }
        };
    if let Some(user_id) = user_id {
        request.extensions_mut().insert(user_id);
        let response = next.run(request).await;
        Ok(response)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

pub async fn make_router(
    db: Db,
    idp_url: String,
    idp_enabled: bool,
) -> Result<Router, Box<dyn Error>> {
    if idp_enabled {
        event!(Level::INFO, "starting server with IDP '{}'", idp_url);
    } else {
        event!(Level::INFO, "starting server without IDP support");
    }
    // Download jwks.json from the passed provider, if IDP auth is enabled.
    let jwks = if idp_enabled {
        let client = reqwest::Client::builder().build()?;
        Some(
            client
                .get(format!("{}/.well-known/jwks.json", idp_url))
                .send()
                .await?
                .json::<jwk::JwkSet>()
                .await?,
        )
    } else {
        None
    };

    let auth_state = AuthState {
        db: db.clone(),
        jwks,
    };

    let authorised = Router::new()
        .route("/recipes", get(get_recipes))
        .route("/recipes", post(create_recipe))
        .route("/recipe/:recipe_id", get(get_recipe))
        .route("/recipe/:recipe_id", put(update_recipe))
        .route("/recipe/:recipe_id", delete(delete_recipe))
        .route("/days", get(get_days))
        .route("/days", post(create_day))
        .route("/days/:date/recipes/:recipe_id", delete(delete_day_recipe))
        .route("/shoppinglist", get(build_list))
        .route("/ingredients", get(get_ingredients))
        .route("/ingredients/user", get(get_user_ingredients))
        .route("/ingredients", post(create_ingredient))
        .route("/ingredient/:ingredient_id", delete(delete_ingredient))
        .route("/schedule", get(get_schedule))
        .route("/schedule", post(create_scheduled_ingredient))
        .route(
            "/schedule/:schedule_id",
            delete(delete_scheduled_ingredient),
        )
        .route("/search/ingredients", get(search_ingredients))
        .layer(middleware::from_fn_with_state(
            auth_state.clone(),
            authorise,
        ))
        .with_state(db);
    let health_check = Router::new().route("/", get(root));
    Ok(Router::new().merge(authorised).merge(health_check))
}

/// A simple 200 for the root of the API
async fn root() -> String {
    "Server started OK.".into()
}

/// Retrieve all the recipes for a given user
async fn get_recipes(
    State(db): State<Db>,
    Extension(user_id): Extension<i32>,
) -> Result<Json<Vec<Recipe>>, ServerError> {
    let result = recipe::query_multiple(&db.pool, user_id).await;

    if result.is_err() {
        event!(Level::ERROR, "{:?}", result);
    }

    result
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

/// Retrieve a single recipe
async fn get_recipe(
    State(db): State<Db>,
    Path(recipe_id): Path<i32>,
    Extension(user_id): Extension<i32>,
) -> Result<Json<Recipe>, ServerError> {
    let result = recipe::query_single(&db.pool, user_id, recipe_id).await;

    if result.is_err() {
        event!(Level::ERROR, "{:?}", result);
    }

    result
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

/// Create a new recipe. Returns the recipe_id of the new recipe.
async fn create_recipe(
    State(db): State<Db>,
    Extension(user_id): Extension<i32>,
    Json(recipe): Json<RecipeInput>,
) -> Result<StatusCode, ServerError> {
    let result = recipe::create(&db.pool, user_id, recipe).await;

    if result.is_err() {
        event!(Level::ERROR, "{:?}", result);
    }

    result
        .map(|_| StatusCode::CREATED)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

/// Update a recipe.
async fn update_recipe(
    State(db): State<Db>,
    Path(recipe_id): Path<i32>,
    Extension(user_id): Extension<i32>,
    Json(recipe): Json<RecipeInput>,
) -> Result<StatusCode, ServerError> {
    let result = recipe::update(&db.pool, user_id, recipe_id, recipe).await;
    if result.is_err() {
        event!(Level::ERROR, "{:?}", result);
    }
    result
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

/// Delete a recipe.
async fn delete_recipe(
    State(db): State<Db>,
    Path(recipe_id): Path<i32>,
    Extension(user_id): Extension<i32>,
) -> Result<StatusCode, ServerError> {
    let result = recipe::delete(&db.pool, user_id, recipe_id).await;

    if result.is_err() {
        event!(Level::ERROR, "{:?}", result);
    }

    result
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

#[derive(Deserialize, Debug)]
struct DayRange {
    from: String,
    to: String,
}
/// Fetch all the day/recipe items for the current user.
async fn get_days(
    State(db): State<Db>,
    Extension(user_id): Extension<i32>,
    Query(params): Query<DayRange>,
) -> Result<Json<Vec<Day>>, ServerError> {
    let result = day::query_range(&db.pool, user_id, &params.from, &params.to).await;

    if result.is_err() {
        event!(Level::ERROR, "{:?}", result);
    }

    result
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

/// Fetch all the day/recipe items for the current user.
async fn build_list(
    State(db): State<Db>,
    Extension(user_id): Extension<i32>,
    Query(params): Query<DayRange>,
) -> Result<Json<ShoppingList>, ServerError> {
    let result = day::build_list_for_range(&db.pool, user_id, params.from, params.to).await;

    if result.is_err() {
        event!(Level::ERROR, "{:?}", result);
    }

    result
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

/// Create a new day/recipe entry in the database
async fn create_day(
    State(db): State<Db>,
    Extension(user_id): Extension<i32>,
    Json(day): Json<DayInput>,
) -> Result<StatusCode, ServerError> {
    let result = day::create(&db.pool, user_id, day).await;
    if result.is_err() {
        event!(Level::ERROR, "{:?}", result);
    }

    result
        .map(|_| StatusCode::CREATED)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

/// Delete a day from the database
async fn delete_day_recipe(
    State(db): State<Db>,
    Extension(user_id): Extension<i32>,
    Path((date, recipe_id)): Path<(String, i32)>,
) -> Result<StatusCode, ServerError> {
    let result = day::delete_day_recipe(&db.pool, user_id, date, recipe_id).await;

    if result.is_err() {
        event!(Level::ERROR, "{:?}", result);
    }

    result
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

/// Fetch all ingredients
async fn get_ingredients(
    State(db): State<Db>,
    Extension(user_id): Extension<i32>,
) -> Result<Json<Vec<Ingredient>>, ServerError> {
    let result = ingredient::query_multiple(&db.pool, user_id).await;

    if result.is_err() {
        event!(Level::ERROR, "{:?}", result);
    }

    result
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

/// Fetch a user's ingredients
async fn get_user_ingredients(
    State(db): State<Db>,
    Extension(user_id): Extension<i32>,
) -> Result<Json<Vec<Ingredient>>, ServerError> {
    let result = ingredient::query_user(&db.pool, user_id).await;

    if result.is_err() {
        event!(Level::ERROR, "{:?}", result);
    }

    result
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

/// Create an ingredient against a user
async fn create_ingredient(
    State(db): State<Db>,
    Extension(user_id): Extension<i32>,
    Json(ingredient): Json<IngredientInput>,
) -> Result<StatusCode, ServerError> {
    let result = ingredient::create(&db.pool, user_id, ingredient).await;

    if result.is_err() {
        event!(Level::ERROR, "{:?}", result);
    }

    result
        .map(|_| StatusCode::CREATED)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

/// Delete an ingredient.
async fn delete_ingredient(
    State(db): State<Db>,
    Path(ingredient_id): Path<i32>,
    Extension(user_id): Extension<i32>,
) -> Result<StatusCode, ServerError> {
    let result = ingredient::delete(&db.pool, user_id, ingredient_id).await;

    if result.is_err() {
        event!(Level::ERROR, "{:?}", result);
    }

    result
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

/// Fetch the user's ingredient schedule
async fn get_schedule(
    State(db): State<Db>,
    Extension(user_id): Extension<i32>,
) -> Result<Json<Vec<ScheduledIngredient>>, ServerError> {
    let result = ingredient::query_scheduled(&db.pool, user_id).await;

    if result.is_err() {
        event!(Level::ERROR, "{:?}", result);
    }

    result
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

/// Create a new recipe. Returns the recipe_id of the new recipe.
async fn create_scheduled_ingredient(
    State(db): State<Db>,
    Extension(user_id): Extension<i32>,
    Json(ingredient): Json<ScheduledIngredientInput>,
) -> Result<StatusCode, ServerError> {
    let result = ingredient::create_scheduled(&db.pool, user_id, ingredient).await;

    if result.is_err() {
        event!(Level::ERROR, "{:?}", result);
    }

    result
        .map(|_| StatusCode::CREATED)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

/// Create a new recipe. Returns the recipe_id of the new recipe.
async fn delete_scheduled_ingredient(
    State(db): State<Db>,
    Path(schedule_id): Path<i32>,
    Extension(user_id): Extension<i32>,
) -> Result<StatusCode, ServerError> {
    let result = ingredient::delete_scheduled(&db.pool, user_id, schedule_id).await;

    if result.is_err() {
        event!(Level::ERROR, "{:?}", result);
    }

    result
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

#[derive(Deserialize)]
struct SearchParams {
    query: String,
}
/// Perform a search over the ingredients table
async fn search_ingredients(
    State(db): State<Db>,
    Query(params): Query<SearchParams>,
    Extension(user_id): Extension<i32>,
) -> Result<Json<Vec<Ingredient>>, ServerError> {
    let result = ingredient::search(&db.pool, user_id, params.query).await;

    if result.is_err() {
        event!(Level::ERROR, "{:?}", result);
    }

    result
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}
