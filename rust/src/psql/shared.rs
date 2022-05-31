use sqlx::types::Uuid;

quick_error! {
    #[derive(Debug, Clone)]
    pub enum LoadError {
        NotFound { }
        DBError(err: String) { }
    }
}

pub fn uuids(ids: &[String]) -> Vec<Uuid> {
    ids.iter().flat_map(|id| Uuid::parse_str(id).ok()).collect()
}
