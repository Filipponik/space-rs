use std::fs;

use mockito::{Matcher, Server};
use serde_json::Value;
use space::Client;

fn load_fixture(fixture_name: &str) -> Value {
    let path = format!("tests/fixtures/{}", fixture_name);
    let content =
        fs::read_to_string(path).expect(&format!("Failed to read fixture: {}", fixture_name));
    serde_json::from_str(&content)
        .expect(&format!("Failed to parse JSON fixture: {}", fixture_name))
}

#[tokio::test]
async fn test_get_issue_for_project_by_number_success() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock(
            "GET",
            "/api/http/projects/id:proj123/planning/issues/number:1",
        )
        .match_query(Matcher::Any)
        .match_header("Authorization", "Bearer test_token")
        .match_header("Accept", "application/json")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(load_fixture("issue_positive_response.json").to_string())
        .create_async()
        .await;

    let client = Client::new(&server.url(), "test_token", None);

    let result = client.get_issue_for_project_by_number("proj123", 1).await;

    mock.assert();

    assert!(result.is_ok());

    let issue = result.unwrap();

    assert_eq!(issue.id, "0198ad97-bb88-7c4b-bbe5-cc0a7878c08f");
    assert_eq!(issue.title, "Test title for first issue");
    assert_eq!(issue.number, 1);
    assert_eq!(issue.status, "Open");
    assert_eq!(
        issue.description,
        Some("Test description for first issue".to_string())
    );

    assert_eq!(issue.created_by.id, "0198ad98-1274-7980-a83a-8e6036fd17bb");
    assert_eq!(issue.created_by.username, "Tester Tester");

    assert!(issue.assignee.is_none());
}

#[tokio::test]
async fn test_get_issue_for_project_by_number_not_found() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock(
            "GET",
            "/api/http/projects/id:proj123/planning/issues/number:999",
        )
        .match_query(Matcher::Any)
        .match_header("Authorization", "Bearer test_token")
        .match_header("Accept", "application/json")
        .with_status(404)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error": "Issue not found"}"#)
        .create_async()
        .await;

    let client = Client::new(&server.url(), "test_token", None);

    let result = client.get_issue_for_project_by_number("proj123", 999).await;

    mock.assert();

    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_issue_for_project_by_number_invalid_json() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock(
            "GET",
            "/api/http/projects/id:proj123/planning/issues/number:42",
        )
        .match_query(Matcher::Any)
        .match_header("Authorization", "Bearer test_token")
        .match_header("Accept", "application/json")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"invalid": "json""#) // Invalid JSON
        .create_async()
        .await;

    let client = Client::new(&server.url(), "test_token", None);

    let result = client.get_issue_for_project_by_number("proj123", 42).await;

    mock.assert();

    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_issue_for_project_by_number_network_error() {
    let client = Client::new("http://127.0.0.1:12345", "test_token", None);
    let result = client.get_issue_for_project_by_number("proj123", 42).await;

    assert!(result.is_err());
}

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

#[tokio::test]
async fn test_get_issues_for_project_success() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("GET", "/api/http/projects/id:proj123/planning/issues")
        .match_query(Matcher::Any)
        .match_header("Authorization", "Bearer test_token")
        .match_header("Accept", "application/json")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(&load_fixture("issues_positive_response.json").to_string())
        .create_async()
        .await;

    let client = Client::new(&server.url(), "test_token", None);
    let query = space::IssuesQuery::new("proj123");

    let result = client.get_issues_for_project(query).await;

    mock.assert();
    assert!(result.is_ok());

    let issues = result.unwrap();
    assert_eq!(issues.len(), 2);

    // Check first issue
    let found_issue1 = issues
        .iter()
        .find(|issue| issue.id == "0198ad97-bb88-7c4b-bbe5-cc0a7878c08f")
        .unwrap();

    assert_eq!(found_issue1.id, "0198ad97-bb88-7c4b-bbe5-cc0a7878c08f");
    assert_eq!(
        found_issue1.description,
        Some("Test description for first issue".to_string())
    );
    assert_eq!(found_issue1.status, "Open");
    assert_eq!(found_issue1.number, 1);
    assert_eq!(
        found_issue1.created_by.id,
        "0198ad98-1274-7980-a83a-8e6036fd17bb"
    );
    assert_eq!(found_issue1.created_by.username, "Tester Tester");
    assert!(found_issue1.assignee.is_none());

    // Check second issue
    let found_issue2 = issues
        .iter()
        .find(|issue| issue.id == "0198ad98-74d8-75db-ae13-ba8265d5caa1")
        .unwrap();

    assert_eq!(found_issue2.id, "0198ad98-74d8-75db-ae13-ba8265d5caa1");
    assert_eq!(
        found_issue2.description,
        Some("Test description for second issue".to_string())
    );
    assert_eq!(found_issue2.status, "Cancelled");
    assert_eq!(found_issue2.number, 2);
    assert_eq!(
        found_issue2.created_by.id,
        "0198ad98-74d8-7a55-8921-486a7e9f4ac5"
    );
    assert_eq!(found_issue2.created_by.username, "Maybe Project Manager");
    assert!(found_issue2.assignee.is_some());
    let issue2_assignee = found_issue2.assignee.clone().unwrap();
    assert_eq!(issue2_assignee.id, "0198ad98-74d8-785e-941b-77f40b4ed03f");
    assert_eq!(issue2_assignee.username, "Best Programmer");
}
