use chrono::DateTime;
use mockito::{Matcher, Server};
use space::{Client, MessagesQuery};

mod utils;

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
        .with_body(&utils::load_fixture("issue_messages_positive_response.json").to_string())
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

#[tokio::test]
async fn test_get_issue_messages_empty_success() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("GET", "/api/http/chats/messages")
        .match_query(Matcher::Any)
        .match_header("Authorization", "Bearer test_token")
        .match_header("Accept", "application/json")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(&utils::load_fixture("issue_messages_positive_empty_response.json").to_string())
        .expect(1)
        .create_async()
        .await;

    let client = Client::new(&server.url(), "test_token", None);
    let message_query = MessagesQuery::new("0198bc7b-ef88-7b76-b9cf-af06e43567ad");
    let result = client.get_issue_messages(message_query).await;

    mock.assert();
    assert!(result.is_ok());
    let messages = result.unwrap();
    assert_eq!(messages.len(), 0);
}
