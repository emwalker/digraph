use std::sync::Arc;

quick_error! {
    #[derive(Debug, Clone)]
    pub enum Error {
        Config(err: envy::Error) {
            from()
        }

        DB(err: Arc<sqlx::Error>) {
            from(err: sqlx::Error) -> (Arc::new(err))
        }

        Deserialization(err: String) {
            from(err: serde_json::Error) -> (format!("{:?}", err))
        }

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

        Utf8(err: std::string::FromUtf8Error) {
            from()
        }
    }
}

impl actix_web::ResponseError for Error {}
