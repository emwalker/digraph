use sqlx::{types::Json, FromRow, PgPool};
use std::collections::HashMap;

use crate::prelude::*;
use crate::schema::{ActivityLineItem, Viewer};

#[derive(FromRow, Clone, Debug)]
pub struct Row {
    created_at: chrono::DateTime<chrono::Utc>,
    url: String,
    title: String,
    user_name: String,
    topics: Json<Vec<HashMap<String, String>>>,
}

impl Row {
    fn to_activity_line_item(&self) -> ActivityLineItem {
        let mut desc = vec![format!(
            "{} added [{}]({})",
            self.user_name, self.title, self.url
        )];

        if !self.topics.is_empty() {
            let mut topics: Vec<String> = vec![];

            for topic in self.topics.iter() {
                let markdown = format!("[{}]({})", topic["name"], topic["resource_path"]);
                topics.push(markdown);
            }

            let topic_string = if topics.len() > 2 {
                let idx = topics.len() - 1;
                format!("{} and {}", topics[..idx].join(", "), topics[idx])
            } else if topics.len() > 1 {
                topics.join(" and ")
            } else {
                topics[0].clone()
            };

            desc.push(format!("and tagged it with {topic_string}"));
        };

        ActivityLineItem {
            description: desc.join(" "),
            created_at: self.created_at,
        }
    }
}

#[allow(dead_code)]
pub struct FetchActivity {
    viewer: Viewer,
    topic_id: Option<String>,
    limit: i32,
}

impl FetchActivity {
    pub fn new(viewer: Viewer, topic_id: Option<String>, first: i32) -> Self {
        Self {
            viewer,
            topic_id,
            limit: first,
        }
    }

    pub async fn call(&self, pool: &PgPool) -> Result<Vec<ActivityLineItem>> {
        let mut index = 2;

        let topic_clause = if self.topic_id.is_some() {
            let clause = format!("ult.topic_id = ${index}::uuid");
            index += 1;
            clause
        } else {
            "true".to_string()
        };

        let query = format!(
            r#"
            select
                ul.created_at,
                l.url,
                l.title,
                u.name user_name,
                json_agg(
                    distinct
                    jsonb_build_object(
                        'name', t.name,
                        'resource_path', concat('/', torg.login, '/topics/', t.id)
                    )
                ) topics

            from user_links ul
            join user_link_topics ult on ul.id = ult.user_link_id
            join links l on l.id = ul.link_id
            join users u on u.id = ul.user_id
            join repositories r on r.id = ul.repository_id
            join organization_members om on om.organization_id = r.organization_id
            join topics t on t.id = ult.topic_id
            join organizations torg on torg.id = t.organization_id

            where om.user_id = any($1::uuid[])
                and {topic_clause}
            group by ul.created_at, l.url, l.title, u.name
            order by ul.created_at desc
            limit ${index}
            "#
        );

        let mut q = sqlx::query_as::<_, Row>(&query).bind(&self.viewer.query_ids);
        if let Some(topic_id) = &self.topic_id {
            q = q.bind(topic_id);
        }
        let rows = q.bind(self.limit).fetch_all(pool).await?;

        Ok(rows.iter().map(Row::to_activity_line_item).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_topics() {
        let topics = serde_json::from_str("[]").unwrap();

        let row = Row {
            created_at: chrono::Utc::now(),
            url: "link".to_string(),
            title: "Link Title".to_string(),
            user_name: "Gnusto".to_string(),
            topics,
        };

        let item = row.to_activity_line_item();
        assert_eq!(item.description, "Gnusto added [Link Title](link)");
    }

    #[test]
    fn test_one_topic() {
        let topics = serde_json::from_str(
            r#"[
                { "name": "Climate change", "resource_path": "/wiki/topics/1" }
            ]"#,
        )
        .unwrap();

        let row = Row {
            created_at: chrono::Utc::now(),
            url: "link".to_string(),
            title: "Link Title".to_string(),
            user_name: "Gnusto".to_string(),
            topics,
        };

        let item = row.to_activity_line_item();
        assert_eq!(
            item.description,
            "Gnusto added [Link Title](link) and tagged it with [Climate change](/wiki/topics/1)"
        );
    }

    #[test]
    fn test_two_topics() {
        let topics = serde_json::from_str(
            r#"[
                { "name": "Climate change", "resource_path": "/wiki/topics/1" },
                { "name": "Biodiversity", "resource_path": "/wiki/topics/2" }
            ]"#,
        )
        .unwrap();

        let row = Row {
            created_at: chrono::Utc::now(),
            url: "link".to_string(),
            title: "Link Title".to_string(),
            user_name: "Gnusto".to_string(),
            topics,
        };

        let item = row.to_activity_line_item();
        assert_eq!(
            item.description,
            "Gnusto added [Link Title](link) and tagged it with [Climate change](/wiki/topics/1) and [Biodiversity](/wiki/topics/2)"
        );
    }

    #[test]
    fn test_three_topics() {
        let topics = serde_json::from_str(
            r#"[
                { "name": "Climate change", "resource_path": "/wiki/topics/1" },
                { "name": "Biodiversity", "resource_path": "/wiki/topics/2" },
                { "name": "Habitat destruction", "resource_path": "/wiki/topics/3" }
            ]"#,
        )
        .unwrap();

        let row = Row {
            created_at: chrono::Utc::now(),
            url: "link".to_string(),
            title: "Link Title".to_string(),
            user_name: "Gnusto".to_string(),
            topics,
        };

        let item = row.to_activity_line_item();
        assert_eq!(
            item.description,
            "Gnusto added [Link Title](link) and tagged it with [Climate change](/wiki/topics/1), [Biodiversity](/wiki/topics/2) and [Habitat destruction](/wiki/topics/3)"
        );
    }
}
