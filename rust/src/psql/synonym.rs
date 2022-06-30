use sqlx::PgPool;
use std::collections::HashSet;

use super::fetch_topic;
use crate::graphql::{alert, Alert, Synonym, Synonyms, Topic, UpdateSynonymsInput, Viewer};
use crate::prelude::*;

pub struct UpdateSynonyms {
    actor: Viewer,
    input: UpdateSynonymsInput,
}

pub struct UpdateSynonymsResult {
    pub alerts: Vec<Alert>,
    pub topic: Topic,
}

impl UpdateSynonyms {
    pub fn new(actor: Viewer, input: UpdateSynonymsInput) -> Self {
        Self { actor, input }
    }

    pub async fn call(&self, pool: &PgPool) -> Result<UpdateSynonymsResult> {
        log::info!("updating synonyms for topic: {:?}", self.input);

        let topic_path = RepoPath::from(&self.input.topic_path);
        // Verify that the user can see the topic
        fetch_topic(&self.actor.mutation_ids, pool, &topic_path)
            .await?
            .to_topic();

        let mut alerts: Vec<Alert> = vec![];
        let mut serialize: Vec<Synonym> = vec![];
        let mut seen: HashSet<&String> = HashSet::new();

        for synonym_input in &self.input.synonyms {
            if seen.contains(&synonym_input.name) {
                continue;
            }
            if synonym_input.is_valid() {
                serialize.push(synonym_input.to_synonym());
            } else {
                alerts.push(alert::warning(format!(
                    "Not a valid name: {}",
                    synonym_input.name
                )));
            }
            seen.insert(&synonym_input.name);
        }

        let topic = fetch_topic(&self.actor.mutation_ids, pool, &topic_path)
            .await?
            .to_topic();

        let synonyms = Synonyms(serialize);
        let synonym_string = serde_json::to_string(&synonyms)?;
        let name = synonyms.display_name("en", &topic.name, &topic.prefix);

        sqlx::query("update topics set name = $1, synonyms = $2::jsonb where id = $3::uuid")
            .bind(&name)
            .bind(&synonym_string)
            .bind(&topic.path.short_id)
            .execute(pool)
            .await?;

        let topic = fetch_topic(&self.actor.mutation_ids, pool, &topic_path)
            .await?
            .to_topic();
        Ok(UpdateSynonymsResult { alerts, topic })
    }
}
