use std::sync::Arc;

quick_error! {
    #[derive(Clone, Debug)]
    pub enum Error {
        Auth(err: String) {}

        Blocking(err: String) {
            from(err: actix_web::error::BlockingError) -> (format!("{}", err))
        }

        Config(err: envy::Error) {
            from()
        }

        Command(err: String) { }

        DB(err: Arc<sqlx::Error>) {
            from(err: sqlx::Error) -> (Arc::new(err))
        }

        DecodeBase64(err: base64::DecodeError) {
            from()
        }

        DecodeEnum(err: strum::ParseError) {
            from()
        }

        Deserialization(err: String) {
            from(err: serde_json::Error) -> (format!("{:?}", err))
        }

        Geotime(err: geotime::Error) {
            from()
        }

        Git2(err: String) {
            from(err: git2::Error) -> (format!("{}", err))
        }

        HeaderValue(err: String) {
            from(err: actix_web::http::header::ToStrError) -> (format!("{:?}", err))
        }

        IO(err: String) {
            from(err: std::io::Error) -> (format!("{}", err))
        }

        Load(err: String) {
            display("problem loading: {}", err)
        }

        NotFound(err: String) {
            display("not found: {}", err)
        }

        Parse(err: String) {}

        Path(err: String) {}

        RBAC(err: String) {}

        Redis(err: String) {
            from(err: redis_rs::RedisError) -> (format!("{}", err))
        }

        Reqwest(err: String) {
            from(err: reqwest::Error) -> (format!("{:?}", err))
        }

        Repo(err: String) { }

        Resolver(err: async_graphql::Error) {
            from()
        }

        SystemTime(err: std::time::SystemTimeError) {
            from()
        }

        UrlParse(err: String) {
            from(err: url::ParseError) -> (format!("{}", err))
        }

        Utf8(err: std::string::FromUtf8Error) {
            from()
        }

        YAML(err: String) {
            from(err: serde_yaml::Error) -> (format!("{}", err))
        }
    }
}

impl actix_web::ResponseError for Error {}
