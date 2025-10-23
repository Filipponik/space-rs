use crate::Member;
use chrono::{DateTime, Utc};
use serde::Deserialize;

pub fn deserialize_created_by<'de, D>(deserializer: D) -> Result<Member, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct CreatedBy {
        name: String,
        details: Option<Details>,
    }

    #[derive(Deserialize)]
    struct Details {
        user: User,
    }

    #[derive(Deserialize)]
    struct User {
        id: String,
    }

    let helper = CreatedBy::deserialize(deserializer)?;
    Ok(Member {
        id: helper
            .details
            .map(|details| details.user.id)
            .unwrap_or_else(|| "deleted".to_string()),
        username: helper.name,
    })
}

pub fn deserialize_space_date<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct SpaceDate {
        iso: DateTime<Utc>,
    }

    let helper = SpaceDate::deserialize(deserializer)?;
    Ok(helper.iso)
}

pub fn deserialize_assignee<'de, D>(deserializer: D) -> Result<Option<Member>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct AssigneeHelper {
        id: String,
        username: String,
    }

    let helper = Option::<AssigneeHelper>::deserialize(deserializer)?;
    Ok(helper.map(|a| Member {
        id: a.id,
        username: a.username,
    }))
}

pub fn deserialize_status<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Status {
        name: String,
    }

    let helper = Status::deserialize(deserializer)?;
    Ok(helper.name)
}

pub fn deserialize_project_key<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Key {
        key: String,
    }

    let helper = Key::deserialize(deserializer)?;
    Ok(helper.key)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use serde_json::json;

    // Helper structures for testing
    #[derive(Deserialize)]
    struct TestCreatedBy {
        #[serde(deserialize_with = "deserialize_created_by")]
        member: Member,
    }

    #[derive(Deserialize)]
    struct TestSpaceDate {
        #[serde(deserialize_with = "deserialize_space_date")]
        date: DateTime<Utc>,
    }

    #[derive(Deserialize)]
    struct TestAssignee {
        #[serde(deserialize_with = "deserialize_assignee", default)]
        assignee: Option<Member>,
    }

    #[derive(Deserialize)]
    struct TestStatus {
        #[serde(deserialize_with = "deserialize_status")]
        status: String,
    }

    #[derive(Deserialize)]
    struct TestProjectKey {
        #[serde(deserialize_with = "deserialize_project_key")]
        key: String,
    }

    #[test]
    fn test_deserialize_created_by_success() {
        let json_data = json!({
            "name": "john_doe",
            "details": {
                "user": {
                    "id": "0198ad98-74d8-7eba-80a2-65f2e3fc2a9d"
                }
            }
        });

        let wrapper: TestCreatedBy = serde_json::from_value(json!({
            "member": json_data
        }))
        .unwrap();

        assert_eq!(wrapper.member.id, "0198ad98-74d8-7eba-80a2-65f2e3fc2a9d");
        assert_eq!(wrapper.member.username, "john_doe");
    }

    #[test]
    fn test_deserialize_space_date_success() {
        let json_data = json!({
            "iso": "2030-10-25T10:30:00Z"
        });

        let wrapper: TestSpaceDate = serde_json::from_value(json!({
            "date": json_data
        }))
        .unwrap();

        let expected = DateTime::parse_from_rfc3339("2030-10-25T10:30:00Z")
            .unwrap()
            .with_timezone(&Utc);
        assert_eq!(wrapper.date, expected);
    }

    #[test]
    fn test_deserialize_assignee_some() {
        let json_data = json!({
            "id": "0198ad98-74d8-7d78-9d4f-518f01e36656",
            "username": "jane_smith"
        });

        let wrapper: TestAssignee = serde_json::from_value(json!({
            "assignee": json_data
        }))
        .unwrap();

        let assignee = wrapper.assignee.unwrap();
        assert_eq!(assignee.id, "0198ad98-74d8-7d78-9d4f-518f01e36656");
        assert_eq!(assignee.username, "jane_smith");
    }

    #[test]
    fn test_deserialize_assignee_none() {
        let json_data = json!(null);

        let wrapper: TestAssignee = serde_json::from_value(json!({
            "assignee": json_data
        }))
        .unwrap();

        assert!(wrapper.assignee.is_none());
    }

    #[test]
    fn test_deserialize_status_success() {
        let json_data = json!({
            "name": "In Progress"
        });

        let wrapper: TestStatus = serde_json::from_value(json!({
            "status": json_data
        }))
        .unwrap();

        assert_eq!(wrapper.status, "In Progress");
    }

    #[test]
    fn test_deserialize_project_key_success() {
        let json_data = json!({
            "key": "PROJ-123"
        });

        let wrapper: TestProjectKey = serde_json::from_value(json!({
            "key": json_data
        }))
        .unwrap();

        assert_eq!(wrapper.key, "PROJ-123");
    }

    #[test]
    fn test_deserialize_assignee_extra_fields() {
        let json_data = json!({
            "id": "0198ad98-74d8-7a6a-8212-c78297ee1c35",
            "username": "jane_smith",
            "email": "jane@example.com"
        });

        let wrapper: TestAssignee = serde_json::from_value(json!({
            "assignee": json_data
        }))
        .unwrap();

        let assignee = wrapper.assignee.unwrap();
        assert_eq!(assignee.id, "0198ad98-74d8-7a6a-8212-c78297ee1c35");
        assert_eq!(assignee.username, "jane_smith");
    }
}
