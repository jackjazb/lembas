use lembas::api::recipe_parser::parse_recipe;

#[tokio::test]
pub async fn test_parse_recipe() {
    let result = parse_recipe("https://www.loveandlemons.com/pasta-salad/").await;
    dbg!(result);
    assert!(false);
}
