use std::fs;

use chrono::DateTime;
use mockito::{Matcher, Server};
use serde_json::Value;
use space::{Client, MessagesQuery};

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

#[tokio::test]
async fn test_get_issue_messages_success() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("GET", "/api/http/chats/messages")
        .match_query(Matcher::Any)
        .match_header("Authorization", "Bearer test_token")
        .match_header("Accept", "application/json")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(&load_fixture("issue_messages_positive_response.json").to_string())
        .expect(1)
        .create_async()
        .await;

    let client = Client::new(&server.url(), "test_token", None);
    let message_query = MessagesQuery::new("0198bc7b-ef88-7b76-b9cf-af06e43567ad");
    let result = client.get_issue_messages(message_query).await;

    mock.assert();
    assert!(result.is_ok());
    let messages = result.unwrap();
    assert_eq!(messages.len(), 5);

    assert_eq!(messages[0].id, "0198bc7b-ef88-792c-8373-7e8ffd64e342");
    assert_eq!(messages[0].text, "created the issue");
    assert_eq!(
        messages[0].created_at,
        DateTime::parse_from_rfc3339("2025-05-28T15:09:34.648Z").unwrap()
    );
    assert_eq!(
        messages[0].author.id,
        "0198bc7b-ef88-7f1e-a96c-eb3e6f8d49ae"
    );
    assert_eq!(messages[0].author.username, "Random tester");

    assert_eq!(messages[1].id, "0198bc7b-ef88-76e0-bf0e-09161e059cd7");
    assert_eq!(messages[1].text, "added the issue to an issue board sprint");
    assert_eq!(
        messages[1].created_at,
        DateTime::parse_from_rfc3339("2025-05-28T15:09:34.649Z").unwrap()
    );
    assert_eq!(
        messages[1].author.id,
        "0198bc7b-ef88-7f1e-a96c-eb3e6f8d49ae"
    );
    assert_eq!(messages[1].author.username, "Random tester");

    assert_eq!(messages[2].id, "0198bc7b-ef88-7de0-b660-3f0af39c616f");
    assert_eq!(messages[2].text, "Assigned to Best Programmer");
    assert_eq!(
        messages[2].created_at,
        DateTime::parse_from_rfc3339("2025-05-29T08:59:36.629Z").unwrap()
    );
    assert_eq!(
        messages[2].author.id,
        "0198bc7b-ef88-7da6-b193-4779e3d23442"
    );
    assert_eq!(messages[2].author.username, "Some teamlead");

    assert_eq!(messages[3].id, "0198bc7b-ef88-7a78-8057-cf942d304de7");
    assert_eq!(messages[3].text, "WTF is this?");
    assert_eq!(
        messages[3].created_at,
        DateTime::parse_from_rfc3339("2025-05-29T10:24:16.023Z").unwrap()
    );
    assert_eq!(
        messages[3].author.id,
        "0198bc7b-ef88-791d-be78-ca843e68e737"
    );
    assert_eq!(messages[3].author.username, "Best Programmer");

    assert_eq!(messages[4].id, "0198bc7b-ef88-725e-a7f2-f6f04f9a5411");
    assert_eq!(messages[4].text, "Status: Open -> Cancelled");
    assert_eq!(
        messages[4].created_at,
        DateTime::parse_from_rfc3339("2025-05-29T10:24:26.267Z").unwrap()
    );
    assert_eq!(
        messages[4].author.id,
        "0198bc7b-ef88-791d-be78-ca843e68e737"
    );
    assert_eq!(messages[4].author.username, "Best Programmer");
}
