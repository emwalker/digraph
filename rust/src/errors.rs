use std::sync::Arc;

quick_error! {
    #[derive(Debug, Clone)]
    pub enum Error {
        Auth(err: String) {}

        Config(err: envy::Error) {
            from()
        }

        DB(err: Arc<sqlx::Error>) {
            from(err: sqlx::Error) -> (Arc::new(err))
        }

        DecodeBase64(err: base64::DecodeError) {
            from()
        }

        Deserialization(err: String) {
            from(err: serde_json::Error) -> (format!("{:?}", err))
        }

        HeaderValue(err: String) {
            from(err: actix_web::http::header::ToStrError) -> (format!("{:?}", err))
        }

        Load(err: String) {
            display("problem loading: {}", err)
        }

        NotFound(err: String) {
            display("not found: {}", err)
        }

        Parse(err: String) {}

        RBAC(err: String) {}

        Reqwest(err: String) {
            from(err: reqwest::Error) -> (format!("{:?}", err))
        }

        Resolver(err: async_graphql::Error) {
            from()
        }

        UrlParse(err: url::ParseError) {
            from()
        }

        Utf8(err: std::string::FromUtf8Error) {
            from()
        }
    }
}

impl actix_web::ResponseError for Error {}
