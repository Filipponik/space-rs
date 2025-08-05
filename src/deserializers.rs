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
        details: Details,
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
        id: helper.details.user.id,
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
