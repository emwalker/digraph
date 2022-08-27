use digraph::config::Config;
use digraph::db;
use digraph::prelude::*;
use sqlx::types::Uuid;
use sqlx::PgPool;

async fn add_private_column_to_repos(pool: &PgPool) -> Result<()> {
    log::info!("adding private column to repositories ...");

    sqlx::query(
        r#"alter table repositories
            add column if not exists
            private bool not null default true"#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"update repositories
            set private = (name = 'system:default')"#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

async fn add_prefix_column_to_repos(pool: &PgPool) -> Result<()> {
    log::info!("adding repo prefixes to repositories ...");

    sqlx::query(
        r#"alter table repositories
            add column if not exists
            prefix text"#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"update repositories
            set prefix = concat('/', o.login, '/')
            from organizations o
            where o.id = repositories.organization_id
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"alter table repositories
            alter column prefix set not null"#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

async fn add_root_topic_path_column_to_repos(pool: &PgPool) -> Result<()> {
    log::info!("adding root topic path column to repositories ...");

    sqlx::query(
        r#"alter table repositories
            add column if not exists
            root_topic_path text"#,
    )
    .execute(pool)
    .await?;

    let (exist,) = sqlx::query_as::<_, (Option<String>,)>("select to_regclass('topics')::text")
        .fetch_one(pool)
        .await?;

    if exist.is_some() {
        log::info!("setting root topics on repositories");

        let rows = sqlx::query_as::<_, (Uuid, String, String)>(
            "select
                r.id,
                r.prefix,
                t.id::text

            from repositories r
            join topics t on r.organization_id = t.organization_id
            where t.root",
        )
        .fetch_all(pool)
        .await?;
        for (repository_id, prefix, _topic_id) in &rows {
            let path = format!("{}{}", prefix, ROOT_TOPIC_ID);

            sqlx::query("update repositories set root_topic_path = $1 where id = $2")
                .bind(&path)
                .bind(&repository_id)
                .execute(pool)
                .await?;
        }
    }

    sqlx::query(
        r#"alter table repositories
            alter column root_topic_path set not null"#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

async fn add_prefix_columns_to_users(pool: &PgPool) -> Result<()> {
    log::info!("adding repo prefixes to users ...");

    sqlx::query(
        r#"alter table users
            add column if not exists
            write_prefixes text[] not null default '{}'"#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"update users
            set write_prefixes = array['/wiki/', concat('/', login, '/')]"#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"alter table users
            add column if not exists
            personal_prefixes text[] not null default '{}'"#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"update users
            set personal_prefixes = array[concat('/', login, '/')]"#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

async fn add_columns_to_organizations(pool: &PgPool) -> Result<()> {
    log::info!("columns to organizations ...");

    sqlx::query(
        "alter table organizations
            add column if not exists owner_id uuid",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "update organizations set owner_id = u.id
            from users u
            where u.login = organizations.login",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "update organizations set owner_id = u.id
            from users u
            where u.login = 'root' and organizations.owner_id is null",
    )
    .execute(pool)
    .await?;

    sqlx::query("alter table organizations alter column owner_id set not null")
        .execute(pool)
        .await?;

    sqlx::query(
        "alter table organizations
            add column if not exists repo_prefix text",
    )
    .execute(pool)
    .await?;

    sqlx::query("update organizations set repo_prefix = concat('/', login, '/')")
        .execute(pool)
        .await?;

    sqlx::query("alter table organizations alter column repo_prefix set not null")
        .execute(pool)
        .await?;

    Ok(())
}

static DROP_COLUMNS: &[(&str, &str)] = &[
    ("repositories", "system"),
    ("organizations", "description"),
    ("organizations", "system"),
    ("organizations", "default_repository_id"),
];

async fn drop_columns(pool: &PgPool) -> Result<()> {
    for (table, column) in DROP_COLUMNS {
        log::info!("dropping column {} from {} table", column, table);
        sqlx::query(&format!(
            "alter table {} drop column if exists {}",
            table, column
        ))
        .execute(pool)
        .await?;
    }

    Ok(())
}

static DROP_ITEMS: &[(&str, &str)] = &[
    ("table", "organization_members"),
    ("table", "topic_transitive_closure"),
    ("table", "topic_topics"),
    ("table", "link_topics"),
    ("view", "user_link_history"),
    ("table", "user_link_topics"),
    ("table", "user_links"),
    ("table", "link_transitive_closure"),
    ("table", "topics"),
    ("table", "timeranges"),
    ("table", "schema_migrations"),
    ("function", "add_topic_to_link"),
    ("function", "add_topic_to_topic"),
    ("function", "link_down_set"),
    ("function", "upsert_topic_down_set"),
    ("function", "upsert_link_down_set"),
    ("function", "topic_upper_set"),
    ("function", "topic_down_set"),
    // First update user_link_reviews
    // "links",
];

async fn drop_items(pool: &PgPool) -> Result<()> {
    for (type_, name) in DROP_ITEMS {
        log::info!("dropping {} {}", type_, name);
        sqlx::query(&format!("drop {} if exists {}", type_, name))
            .execute(pool)
            .await?;
    }

    Ok(())
}

struct Opts {
    destructive_migrations: bool,
}

fn parse_args() -> Opts {
    let args: Vec<String> = std::env::args().collect();

    let mut opts = getopts::Options::new();
    opts.optflag("d", "destructive", "run destructive migrations");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };

    Opts {
        destructive_migrations: matches.opt_present("d"),
    }
}

#[actix_web::main]
async fn main() -> async_graphql::Result<()> {
    let config = Config::load()?;
    env_logger::init();

    let opts = parse_args();

    let pool = db::db_connection(&config).await?;
    add_private_column_to_repos(&pool).await?;
    add_prefix_column_to_repos(&pool).await?;
    add_root_topic_path_column_to_repos(&pool).await?;
    add_prefix_columns_to_users(&pool).await?;
    add_columns_to_organizations(&pool).await?;

    if opts.destructive_migrations {
        log::info!("running destructive migrations");
        drop_columns(&pool).await?;
        drop_items(&pool).await?;
    }

    log::info!("database migrated.");
    Ok(())
}
