use mockito::{Matcher, Server};
use space::Client;
use utils::load_fixture;

mod utils;

#[tokio::test]
async fn test_get_projects_success() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("GET", "/api/http/projects")
        .match_query(Matcher::Any)
        .match_header("Authorization", "Bearer test_token")
        .match_header("Accept", "application/json")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(&load_fixture("projects_positive_response.json").to_string())
        .create_async()
        .await;

    let client = Client::new(&server.url(), "test_token", None);
    let result = client.get_projects().await;

    mock.assert();
    assert!(result.is_ok());

    let projects = result.unwrap();
    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].id, "0198ad98-74d8-7235-a4c6-f0a5368a1fb6");
    assert_eq!(projects[0].name, "Test Project");
    assert_eq!(projects[0].key, "TEST1");
}
