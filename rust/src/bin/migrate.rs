use digraph::config::Config;
use digraph::db;
use digraph::prelude::*;
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
            set private = (name = 'Personal repo')"#,
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

    Ok(())
}

async fn add_user_repositories(pool: &PgPool) -> Result<()> {
    sqlx::query(
        "create table if not exists users_repositories (
            user_id uuid not null references users (id),
            repository_id uuid not null references repositories (id),
            can_read boolean not null default 'f',
            can_write boolean not null default 'f',
            is_personal_repo boolean not null default 'f',
            primary key (user_id, repository_id)
        )",
    )
    .execute(pool)
    .await?;

    // Add everyone to the wiki repo
    sqlx::query(
        "insert into users_repositories (user_id, repository_id, can_read, can_write)
            select u.id, r.id, 't', 't'
            from users u
            cross join repositories r
            join organizations o on r.organization_id = o.id
            where o.login = 'wiki'
            on conflict do nothing",
    )
    .execute(pool)
    .await?;

    // Give people access to their own private repos
    sqlx::query(
        "insert into users_repositories (user_id, repository_id, can_read, can_write,
                is_personal_repo)
            select u.id, r.id, 't', 't', 't'
            from users u
            join repositories r on u.id = r.owner_id
            on conflict do nothing",
    )
    .execute(pool)
    .await?;

    Ok(())
}

async fn update_repo_and_org_names(pool: &PgPool) -> Result<()> {
    log::info!("updating repo and org names ...");

    sqlx::query("update repositories set name = 'Personal repo' where name = 'system:default'")
        .execute(pool)
        .await?;

    sqlx::query("update repositories set name = 'Wiki' where name = 'General collection'")
        .execute(pool)
        .await?;

    sqlx::query("update organizations set name = 'Wiki' where name = 'General'")
        .execute(pool)
        .await?;

    sqlx::query("update organizations set name = 'Personal org' where name = 'system:default'")
        .execute(pool)
        .await?;

    Ok(())
}

async fn update_personal_repos(pool: &PgPool) -> Result<()> {
    sqlx::query("update repositories set private = 't' where id <> $1::uuid")
        .bind(WIKI_REPOSITORY_ID)
        .execute(pool)
        .await?;

    Ok(())
}

static DROP_COLUMNS: &[(&str, &str)] = &[
    ("repositories", "system"),
    ("organizations", "description"),
    ("organizations", "system"),
    // ("organizations", "default_repository_id"),
];

async fn drop_columns(pool: &PgPool) -> Result<()> {
    for (table, column) in DROP_COLUMNS {
        log::info!("dropping column {} from {} table", column, table);
        sqlx::query(&format!(
            "alter table {table} drop column if exists {column}"
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
        sqlx::query(&format!("drop {type_} if exists {name}"))
            .execute(pool)
            .await?;
    }

    Ok(())
}

async fn delete_migration_records(pool: &PgPool) -> Result<()> {
    log::info!("deleting rows from _sqlx_migrations");
    sqlx::query("delete from _sqlx_migrations")
        .execute(pool)
        .await?;

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
    let config = Config::load().unwrap();
    env_logger::init();

    let opts = parse_args();

    let pool = db::db_connection(&config).await.unwrap();
    add_private_column_to_repos(&pool).await.unwrap();
    add_columns_to_organizations(&pool).await.unwrap();
    add_user_repositories(&pool).await.unwrap();
    update_repo_and_org_names(&pool).await.unwrap();
    update_personal_repos(&pool).await.unwrap();

    if opts.destructive_migrations {
        log::info!("running destructive migrations");
        drop_columns(&pool).await.unwrap();
        drop_items(&pool).await.unwrap();
    }

    delete_migration_records(&pool).await.unwrap();

    log::info!("database migrated.");
    Ok(())
}
