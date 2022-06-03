use sqlx::types::Uuid;

pub fn uuids(ids: &[String]) -> Vec<Uuid> {
    ids.iter().flat_map(|id| Uuid::parse_str(id).ok()).collect()
}
