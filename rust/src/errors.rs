quick_error! {
    #[derive(Debug, Clone)]
    pub enum Error {
        NotFound {}
        Load {}
        Resolver(err: async_graphql::Error) {
            from()
        }
        DB {
            from(sqlx::Error)
        }
    }
}

impl actix_web::ResponseError for Error {}
