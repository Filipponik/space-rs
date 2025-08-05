mod deserializers;

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub struct Client {
    #[allow(clippy::struct_field_names)]
    http_client: reqwest::Client,
    base_url: String,
    auth_token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Member {
    id: String,
    username: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Issue {
    pub id: String,
    pub title: String,
    pub number: usize,
    #[serde(
        rename(deserialize = "createdBy"),
        deserialize_with = "deserializers::deserialize_created_by"
    )]
    pub created_by: Member,
    #[serde(deserialize_with = "deserializers::deserialize_assignee", default)]
    pub assignee: Option<Member>,
    #[serde(deserialize_with = "deserializers::deserialize_status")]
    pub status: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub id: String,
    pub text: String,
    #[serde(deserialize_with = "deserializers::deserialize_created_by")]
    pub author: Member,
    #[serde(
        rename(deserialize = "created"),
        deserialize_with = "deserializers::deserialize_space_date"
    )]
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuesApiResponse {
    data: Vec<Issue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MessagesApiResponse {
    messages: Vec<Message>,
    #[serde(rename(deserialize = "nextStartFromDate"))]
    next_start_from_date: Option<PaginationDate>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PaginationDate {
    iso: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuesQuery {
    #[serde(skip_serializing)]
    pub project_id: String,
    pub sorting: String,
    pub descending: bool,
    #[serde(rename(serialize = "$top"))]
    pub top: usize,
    #[serde(rename(serialize = "$skip"))]
    pub skip: usize,
    #[serde(rename(serialize = "$fields"))]
    pub fields: String,
}

impl IssuesQuery {
    pub fn new(project_id: &str) -> Self {
        Self {
            project_id: project_id.to_string(),
            sorting: "CREATED".to_string(),
            descending: true,
            top: 1000,
            skip: 0,
            fields: "data(assignee(username,id),id,number,status,title,description,createdBy,commentsCount),next,totalCount".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessagesQuery {
    pub channel: String,
    pub sorting: String,
    #[serde(rename(serialize = "batchSize"))]
    pub batch_size: usize,
    #[serde(rename(serialize = "$fields"))]
    pub fields: String,
    #[serde(rename(serialize = "startFromDate"))]
    pub start_from_date: Option<DateTime<Utc>>,
}

impl MessagesQuery {
    pub fn new(issue_id: &str) -> Self {
        Self {
            channel: format!("issue:id:{issue_id}"),
            sorting: "FromNewestToOldest".to_string(),
            batch_size: 50,
            fields: "nextStartFromDate,orgLimitReached,messages(id,author,created,text)"
                .to_string(),
            start_from_date: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    id: String,
    name: String,
    #[serde(deserialize_with = "deserializers::deserialize_project_key")]
    key: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectsApiResponse {
    data: Vec<Project>,
}

impl Client {
    pub fn new(base_url: &str, auth_token: &str, http_client: Option<reqwest::Client>) -> Self {
        Self {
            http_client: http_client.unwrap_or_default(),
            base_url: base_url.to_string(),
            auth_token: auth_token.to_string(),
        }
    }

    pub async fn get_projects(&self) -> Vec<Project> {
        let url = format!("{}/api/http/projects", self.base_url);

        self.send_request::<_, ProjectsApiResponse>(&url, ())
            .await
            .data
    }

    pub async fn get_issues_for_project(&self, query: IssuesQuery) -> Vec<Issue> {
        let url = format!(
            "{}/api/http/projects/id:{}/planning/issues",
            self.base_url, query.project_id
        );

        self.send_request::<_, IssuesApiResponse>(&url, query)
            .await
            .data
    }

    pub async fn get_issue_for_project_by_number(&self, project_id: &str, number: u32) -> Issue {
        let url = format!(
            "{}/api/http/projects/id:{project_id}/planning/issues/number:{number}",
            self.base_url
        );

        let query = HashMap::from([(
            "$fields",
            "assignee(username,id),id,number,status,title,description,createdBy,commentsCount",
        )]);

        self.send_request::<_, Issue>(&url, query).await
    }

    pub async fn get_issue_messages(&self, query: MessagesQuery) -> Vec<Message> {
        let url = format!("{}/api/http/chats/messages", self.base_url);

        let mut actual_query = query.clone();
        let mut messages: HashMap<String, Message> = HashMap::new();
        loop {
            let response = self
                .send_request::<_, MessagesApiResponse>(&url, actual_query.clone())
                .await;

            if response.messages.is_empty() {
                break;
            }

            response.messages.into_iter().for_each(|message| {
                messages.insert(message.id.clone(), message);
            });

            match response.next_start_from_date {
                Some(date) => {
                    actual_query.start_from_date = Some(date.iso);
                }
                None => {
                    break;
                }
            }
        }

        let mut res = messages.into_values().collect::<Vec<Message>>();
        res.sort_by(|a, b| a.created_at.cmp(&b.created_at));
        res
    }

    async fn send_request<TQuery, TResponse>(&self, url: &str, query: TQuery) -> TResponse
    where
        TQuery: Serialize + Send,
        TResponse: for<'de> Deserialize<'de> + Send,
    {
        let result = self
            .http_client
            .get(url)
            .header("Authorization", format!("Bearer {}", self.auth_token))
            .header("Accept", "application/json")
            .query(&query)
            .send()
            .await
            .expect("Failed to send request");

        result.json().await.expect("Failed to parse response")
    }
}
