use std::sync::Arc;

quick_error! {
    #[derive(Clone, Debug)]
    pub enum Error {
        Auth(err: String) {}

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
            from(err: serde_json::Error) -> (format!("{err:?}"))
        }

        Geotime(err: geotime::Error) {
            from()
        }

        Git2(err: String) {
            from(err: git2::Error) -> (format!("{err}"))
        }

        IO(err: String) {
            from(err: std::io::Error) -> (format!("{err}"))
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
            from(err: redis_rs::RedisError) -> (format!("{err}"))
        }

        Repo(err: String) { }

        Resolver(err: async_graphql::Error) {
            from()
        }

        SystemTime(err: std::time::SystemTimeError) {
            from()
        }

        Ureq(err: String) {
            from(err: ureq::Error) -> (format!("{err}"))
        }

        UrlParse(err: String) {
            from(err: url::ParseError) -> (format!("{err}"))
        }

        Utf8Error(err: std::str::Utf8Error) {
            from()
        }

        Utf8(err: std::string::FromUtf8Error) {
            from()
        }

        Uuid(err: uuid::Error) {
            from()
        }

        YAML(err: String) {
            from(err: serde_yaml::Error) -> (format!("{err}"))
        }
    }
}
