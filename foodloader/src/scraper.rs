use rand::Rng;
use regex::Regex;
use reqwest::{header, Client};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{error::Error, fs::File, io::Write, u64::MAX};

pub const INGREDIENTS_FILE: &str = "data/ingredients.json";

const RESPONSES_FILE: &str = "data/responses.json";

const CHROME_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
    AppleWebKit/537.36 (KHTML, like Gecko) \
    Chrome/119.0.0.0 Safari/537.36";

const UNITS: [&str; 7] = ["Punnet", "Loaf", "g", "ml", "kg", "L", "x"];

// Stores a unit and amount.
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Ingredient {
    pub id: i32,
    pub name: String,
    pub unit: Option<String>,
    pub minimum_quantity: f64,
    pub purchase_quantity: f64,
    pub life: i32,
}

// Currently restricted to Sainsbury's own brand items.
fn build_api_request(category_id: String, page_number: u64) -> String {
    let nav_brand = "sainsbury%27s%2Csainsbury%27s";
    format!(
        "https://www.sainsburys.co.uk/groceries-api/gol-services/product/v1\
        /product?filter[keyword]=&filter[category]={category_id}\
        &browse=false&hfss_restricted=false&filter[nav_Brand]={nav_brand}\
        &sort_order=FAVOURITES_FIRST&include[PRODUCT_AD]=citrus&page_number={page_number}\
        &page_size=90&citrus_placement=category-only"
    )
}

/// Scrapes Sainsbury's internal product search API for each ingredient in a given dataset.
pub async fn scrape() -> Result<(), Box<dyn Error>> {
    let client = build_client()?;

    // The category IDs to scrape (label id lifetime).
    // Order is important, as we want to make sure lifetime estimates are as accurate as possible -
    // if an ingredient appears in 'Milk' and 'Baking', we take the first category which has a shorter life.
    let category_ids = vec![
        ("Butter", "428877", 60),
        ("Milk", "428869", 7),
        ("Meat and Fish Essentials", "474595", 7),
        ("Cheese", "428899", 7),
        ("Eggs", "428936", 14),
        ("Dairy Free", "428940", 7),
        ("Fresh Fruit", "12545", 7),
        ("Fresh Vegetables", "13237", 7),
        ("Fresh Herbs", "258774", 7),
        ("Bread", "272787", 7),
        ("Herbs and Spices", "364401", 365),
        ("Pulses and Beans", "13263", 365),
        ("Flour", "514859", 365),
        ("Salt", "13153", 365),
        ("Oil", "267545", 365),
        ("Rice", "243754", 365),
        ("Pasta", "276023", 365),
        ("Noodles", "12595", 365),
        ("Stock", "13158", 365),
        ("Baking Essentials", "318452", 365),
        ("Packaged Tomatoes", "13254", 365),
    ];

    let mut ingredients: Vec<Ingredient> = vec![];
    let mut current_id = 1;

    // Record responses to disk - might as well, and will have a contingency if the API is shut down.
    let mut responses: Vec<serde_json::Value> = vec![];

    // To avoid duplicate products
    let mut seen_uids: Vec<String> = vec![];

    let mut rng = rand::thread_rng();

    for (label, id, life) in category_ids {
        println!("\n-> Scraping \"{}\"", label);

        let mut page: u64 = 1;
        let mut max_page: u64 = MAX;

        while page <= max_page {
            println!("-> Page {}", page);
            // Build the URL to query
            let url = build_api_request(String::from(id), page);
            page += 1;

            // Get all Sainsbury's own brand foods for each category
            let response = client
                .get(url)
                .send()
                .await?
                .json::<serde_json::Value>()
                .await?;

            // Pagination discovered in first request
            if let Some(pages) = response
                .get("controls")
                .and_then(|a| a.get("page"))
                .and_then(|a| a.get("last"))
                .and_then(|a| a.as_number())
                .and_then(|a| a.as_u64())
            {
                max_page = pages;
            } else {
                max_page = 1;
            }
            if let Some(products) = response.get("products").and_then(|a| a.as_array()) {
                let ingredients_initial_len = ingredients.len();
                if products.len() == 0 {
                    println!("   ✗ Nothing returned");
                }

                // Print a random example
                if products.len() > 1 {
                    if let Some(example) = products
                        .get(rng.gen_range(0..products.len() - 1))
                        .and_then(|p| p.get("name"))
                        .and_then(|p| p.as_str())
                    {
                        println!("   + E.g. \"{}\"", example);
                    }
                }
                // Save the raw response data.
                responses.push(response.clone());

                // Process each product.
                for product in products {
                    if let Some((ingredient, uid)) = process_product(product.clone()) {
                        if seen_uids.contains(&uid) {
                            println!("   - Duplicate skipped: \"{}\"", ingredient.name);
                            continue;
                        }
                        seen_uids.push(uid);
                        let mut ingredient_mod = ingredient.clone();
                        ingredient_mod.id = current_id;
                        ingredient_mod.life = life;
                        current_id += 1;
                        ingredients.push(ingredient_mod);
                    } else {
                        println!("   - Item skipped: invalid or filtered out.")
                    }
                }
                let added = ingredients.len() - ingredients_initial_len;
                println!("   ✓ Category contained {} ingredients", added);
            } else {
                println!("   ✗ Request failed or returned erroneous data.")
            }
        }
    }
    println!(
        "\nWriting {} ingredients to '{}'...",
        ingredients.len(),
        INGREDIENTS_FILE
    );
    let j = serde_json::to_string(&ingredients)?;
    let mut ingredients = File::create(INGREDIENTS_FILE)?;
    ingredients.write_all(j.as_bytes())?;

    println!(
        "Writing {} responses to '{}'...\n",
        responses.len(),
        RESPONSES_FILE
    );
    let j = serde_json::to_string(&responses)?;
    let mut responses = File::create(RESPONSES_FILE)?;
    responses.write_all(j.as_bytes())?;

    Ok(())
}

/// Process a product into an ingredient. Takes a JSON value and set of existing UIDs for debugging purposes.
/// Returns an ingredient and a set of additional UIDs.
fn process_product(product: Value) -> Option<(Ingredient, String)> {
    // If any of these keywords are present, the product is dropped.
    // Includes things like non-edible items and ready meals.
    let ignore_if_present = vec![
        "home",
        "basics",
        "pizza",
        "ravioli",
        "tortelloni",
        "ready",
        "quiche",
        "with",
        "pie",
        "kitchen",
        "grinder",
        "mashed",
        "on the go",
        "mix ",
        "quick cook",
        "microwave",
        "tortell",
        "medley",
        "tartlets",
        "pastry case",
        "paper",
        "rich",
        "datterini",
        "4x400g",
        "4×400g",
    ];

    let name = product
        .get("name")
        .and_then(|d| d.as_str())
        .map(|e| e.to_string())?;

    let uid = product
        .get("product_uid")
        .and_then(|d| d.as_str())
        .map(|e| e.to_string())?;

    for word in ignore_if_present {
        if name.to_lowercase().contains(word) {
            return None;
        }
    }

    // Clean up the product name string.
    let mut final_name = clean_product_string(&name);

    let unit_filter = final_name.clone();
    // Check each word of the product name to find a unit string
    let amount_strings = unit_filter
        .split(" ")
        .into_iter()
        .filter(|&s| is_amount_string(s.into()))
        .collect::<Vec<_>>();

    if let Some(&amount_string) = amount_strings.get(0) {
        // Remove the amount string from the product name and reformat
        final_name = final_name.replace(amount_string, "");
        final_name = clean_product_string(&final_name);
        if let Some((amount, unit)) = get_amount_unit(amount_string.into()).ok() {
            final_name = strip_unit_strings(&final_name);
            return Some((
                Ingredient {
                    id: uid.parse().unwrap(),
                    name: final_name,
                    unit: unit.clone(),
                    minimum_quantity: 1.0,
                    purchase_quantity: amount.clone(),
                    life: 7,
                },
                uid,
            ));
        }
        return None;
    }

    // If no amount is found, assume product is bought in single quantities.
    Some((
        Ingredient {
            id: uid.parse().unwrap(),
            name: final_name,
            unit: None,
            minimum_quantity: 1.0,
            purchase_quantity: 1.0,
            life: 7,
        },
        uid,
    ))
}

/// Removes all unit strings from a string
fn strip_unit_strings(string: &String) -> String {
    string
        .split(' ')
        .filter(|w| !is_amount_string(w.to_string()))
        .collect::<Vec<_>>()
        .join(" ")
}
/// Tidies up product strings
fn clean_product_string(name: &String) -> String {
    // Pointless branding words - filtered out.
    let ignored_words = vec![
        "Sainsbury's",
        "Sainsbury’s",
        "Sainsburys",
        "Yeo Valley",
        "Somerset",
        "Inspired to Cook",
        "SO Organic",
        "So Organic",
        "So ",
        "JS",
        "British or Irish",
        "British or New Zealand",
        "British",
        "Irish",
        "Scottish",
        "Italian",
        "Hand-Crafted",
        "Taste The Difference",
        "Taste the Difference",
        "Taste Difference",
        "Mini",
        "Min ",
        "Butcher's choice",
        "Cumberland",
        "MSC",
        "ASC",
        "Fairtrade",
        "Be Good To Yourself",
        "Autumn Edition",
        "Loose",
        "Vittoria",
        "Bunched",
        "Bunch",
        "Ripe & Ready",
        "Seedless",
        "Sundream",
        "Pomodorino",
        "Premium",
        "Everyday",
        "minimum",
        "Unwaxed",
        "Organic",
        "Wafer Thin Air Dried ",
        "Deliciously Free From",
        "Free From",
        "FreeFrom",
        "Alternative To Cheese",
        "Original",
        "Polybag",
        "Unbleached",
    ];

    // Remove commas and non-ascii spaces
    let mut cleaned = name.replace(",", "");
    cleaned = cleaned.replace(" ", " ");

    // Filter out words in brackets (these are usually erroneous)
    let re = Regex::new(r"\((.*?)\)").unwrap();
    cleaned = re.replace_all(&cleaned, "").to_string();

    // Filter out ignored words
    for word in &ignored_words {
        cleaned = cleaned.replace(word, "");
    }

    // Filter out extra spaces
    cleaned
        .trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

/// Splits a string into unit and amount (e.g. 200g -> (200.0, Some("g"))).
fn get_amount_unit(string: String) -> Result<(f64, Option<String>), Box<dyn Error>> {
    // Remove all numeric and symbol characters and return true if only a unit remains
    let maybe_unit: String = string
        .chars()
        .into_iter()
        .filter(|c| c.is_alphabetic())
        .collect();

    // Leaves unitless strings unchanged.
    let mut amount: f64 = string.replace(&maybe_unit, "").parse()?;

    // "x" is a special case - should be translated to indicate whole items.
    let unit = match UNITS.contains(&maybe_unit.as_str()) {
        true => {
            if maybe_unit == "x" {
                None
            } else if maybe_unit == "L" {
                amount *= 1000.0;
                Some("ml".into())
            } else if maybe_unit == "kg" {
                amount *= 1000.0;
                Some("g".into())
            } else {
                Some(maybe_unit)
            }
        }
        false => None,
    };

    Ok((amount, unit))
}

/// True if a string is parseable as an amount.
fn is_amount_string(string: String) -> bool {
    get_amount_unit(string).ok().is_some()
}

/// Builds an HTTP client with spoofed headers and user agent.
fn build_client() -> Result<Client, Box<dyn Error>> {
    let headers = vec![
        (
            "Accept",
            "application/json,text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8",
        ),
        ("Accept-Language", "en-US,en;q=0.5"),
        ("Accept-Encoding", "gzip, deflate"),
        ("Connection", "keep-alive"),
        ("Upgrade-Insecure-Requests", "1"),
        ("Sec-Fetch-Dest", "document"),
        ("Sec-Fetch-Mode", "navigate"),
        ("Sec-Fetch-Site", "none"),
        ("Sec-Fetch-User", "?1"),
        ("Cache-Control", "max-age=0"),
		("Enabled-Feature-Flags","ads_conditionals,findability_v5,show_static_cnc_messaging,event_dates,fetch_future_slot_weeks,click_and_collect_promo_banner,cookie_law_link,citrus_banners,citrus_search_trio_banners,citrus_favourites_trio_banners,offers_trio_banners_single_call,special_logo,custom_product_messaging,promotional_link,findability_search,findability_autosuggest,fto_header_flag,recurring_slot_skip_opt_out,seasonal_favourites,cnc_start_amend_order_modal,favourites_product_cta_alt,get_favourites_from_v2,saver_slot_update,offers_config,alternatives_modal,relevancy_rank,show_hd_xmas_slots_banner,favourites_pill_nav,nectar_destination_page,nectar_card_associated,browse_pills_nav_type,zone_featured,meal_planner_noindex,use_cached_findability_results,event_zone_list,call_bcs,show_ynp_change_slot_banner,event_carousel_skus,catchweight_dropdown,citrus_xsell,constant_commerce_v2,desktop_interstitial_variant,disable_product_cache_validation,favourites_whole_service,first_favourites_static,foodmaestro_modal,hfss_restricted,interstitial_variant,kg_price_label,krang_recommendations,mobile_interstitial_variant,nectar_prices,new_favourites_service,ni_brexit_banner,recipes_ingredients_modal,review_syndication,sale_january,xmas_dummy_skus,your_nectar_prices")
    ];
    let mut header_map = header::HeaderMap::new();

    for header in headers.iter() {
        header_map.insert(header.0, header::HeaderValue::from_static(header.1));
    }

    Ok(reqwest::Client::builder()
        .user_agent(CHROME_USER_AGENT)
        .default_headers(header_map)
        .gzip(true)
        .build()?)
}
