use repository_wrapup::github;

#[tokio::test]
async fn test_get_single_repo() {
    let result =
        github::get_single_repo(&github::octocrab_handle(), "manchicken", "manchicken").await;

    assert!(result.is_ok());
    let result_value = result.unwrap();
    assert!(result_value.name.eq(&"manchicken".to_string()));
    println!("Result: {:#?}", result_value);
}
