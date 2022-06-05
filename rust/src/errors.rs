use std::sync::Arc;

quick_error! {
    #[derive(Debug, Clone)]
    pub enum Error {
        Load(err: String) {
            display("problem loading: {}", err)
        }

        NotFound(err: String) {
            display("not found: {}", err)
        }

        Parse(err: String) {}

        Resolver(err: async_graphql::Error) {
            from()
        }

        DB(err: Arc<sqlx::Error>) {
            from(err: sqlx::Error) -> (Arc::new(err))
        }
    }
}

impl actix_web::ResponseError for Error {}
