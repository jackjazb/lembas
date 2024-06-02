use std::error::Error;

use axum::{extract::Query, http::StatusCode, Extension, Json};
use tracing::error;

use crate::service::ingredient::Ingredient;

use super::router::ServerError;

const LD_JSON_SELECTOR: &str = "script[type=\"application/ld+json\"]";
const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36";

#[derive(serde::Deserialize)]
pub struct ParseQuery {
    url: String,
}

/// Handler for parsing recipes.
pub async fn parse_handler(
    Extension(account_id): Extension<i32>,
    Query(query): Query<ParseQuery>,
) -> Result<Json<ParsedRecipe>, ServerError> {
    let result = parse_recipe(&query.url).await;
    if result.is_err() {
        error!("{:?}", result);
    }

    result
        .map(|v| Json(v))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

/**
 * Notes
 * - Want to fuzzy search over ingredients here - so we can return the website ingredient and a match if found
 */
/// A subset of the Recipe schema found at [schema.org](https://schema.org/Recipe).
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct LinkedDataRecipe {
    name: String,
    #[serde(rename(deserialize = "recipeYield"))]
    ld_yield: Yield,
    #[serde(rename(deserialize = "recipeIngredient"))]
    ingredients: Vec<String>,
    #[serde(rename(deserialize = "recipeInstructions"))]
    steps: Vec<Step>,
}

/// Recipe steps can be either a string or an object.
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[serde(untagged)]
enum Step {
    String(String),
    Object { text: String },
}

/// Recipe steps can be either a string or an object.
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[serde(untagged)]
enum Yield {
    String(String),
    Array(Vec<String>),
}

impl LinkedDataRecipe {
    fn from_html(html: &str) -> Option<Self> {
        // Some recipes encapcsulate LD data in a graph with multiple objects.
        #[derive(serde::Deserialize, Debug, Clone)]
        pub struct LinkedDataGraph {
            #[serde(rename(deserialize = "@graph"))]
            graph: Vec<serde_json::Value>,
        }

        let as_recipe =
            |string: &str| -> serde_json::Result<LinkedDataRecipe> { serde_json::from_str(string) };

        let as_graph =
            |string: &str| -> serde_json::Result<LinkedDataGraph> { serde_json::from_str(string) };

        // Parse the passed HTML with `scraper`
        let document = scraper::Html::parse_document(&html);
        let selector = scraper::Selector::parse(LD_JSON_SELECTOR).unwrap();

        // Check each script tag
        for element in document.select(&selector) {
            let json = &element.inner_html();
            let graph_result = as_graph(json);
            if let Ok(graph) = graph_result {
                for obj in graph.graph {
                    let graph_item_result = serde_json::from_value::<LinkedDataRecipe>(obj);
                    dbg!(&graph_item_result);
                    if let Ok(recipe) = graph_item_result {
                        return Some(recipe);
                    }
                }
            }
            let recipe_result = as_recipe(json);
            if let Ok(recipe) = recipe_result {
                return Some(recipe);
            }
        }
        None
    }

    /// Extract a number from the `yield` field of the recipe. Returns 1 if unclear.
    pub fn get_int_portions(&self) -> i32 {
        // let text = match &self.ld_yield {
        //     Yield::String(s) => s,
        //     Yield::Array(v) => v.get(0).unwrap_or(&String::new()),
        // };
        // let words = self
        //     .ld_yield
        //     .split(" ")
        //     .filter(|w| w.parse::<i32>().is_ok())
        //     .collect::<Vec<_>>();
        // words[0].parse::<i32>().unwrap();
        2
    }
}

/// The output of a successfully parsed web recipe.
#[derive(serde::Serialize, Debug)]
pub struct ParsedRecipe {
    title: String,
    portions: i32,
    steps: Vec<String>,
    ingredients: Vec<MatchedIngredient>,
}

/// Represents an ingredient from a web recipe with a possible match from the
/// database if one could be found.
#[derive(serde::Serialize, Debug)]
pub struct MatchedIngredient {
    ingredient: String,
    matched: Option<Ingredient>,
}

impl From<LinkedDataRecipe> for ParsedRecipe {
    fn from(value: LinkedDataRecipe) -> Self {
        let title = value.clone().name;
        let portions = value.get_int_portions();
        let steps = value
            .steps
            .iter()
            .map(|s| {
                match s {
                    Step::String(s) => s,
                    Step::Object { text } => text,
                }
                .to_owned()
            })
            .collect::<Vec<String>>();
        let ingredients = vec![MatchedIngredient {
            ingredient: "test".into(),
            matched: None,
        }];
        ParsedRecipe {
            title,
            portions,
            steps,
            ingredients,
        }
    }
}

/// Takes a URL of a web page and attempts to extract a JSON linked data recipe from it.
pub async fn parse_recipe(url: &str) -> Result<ParsedRecipe, Box<dyn Error>> {
    let client = reqwest::Client::builder()
        .gzip(true)
        .user_agent(USER_AGENT)
        .build()?;

    let res = client
        .get(url)
        .header("Accept", "*/*")
        .header("Accept-Encoding", "gzip")
        .send()
        .await?;
    let body = res.text().await?;
    let ld_recipe = LinkedDataRecipe::from_html(&body);
    dbg!(&ld_recipe);

    if let Some(recipe) = ld_recipe {
        return Ok(recipe.into());
    }
    Err("no recipes on page".into())
}
