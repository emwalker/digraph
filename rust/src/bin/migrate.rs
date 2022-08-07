use digraph::config::Config;
use digraph::db;
use digraph::prelude::*;
use digraph::types::sha256_base64;
use sqlx::types::Uuid;
use sqlx::PgPool;

// TODO: Drop organization_members (?)

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
            set private = (system and name = 'system:default')"#,
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

    for (repository_id, prefix, topic_id) in &rows {
        let hashed = sha256_base64(topic_id);
        let path = format!("{}{}", prefix, hashed);

        sqlx::query("update repositories set root_topic_path = $1 where id = $2")
            .bind(&path)
            .bind(&repository_id)
            .execute(pool)
            .await?;
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

#[actix_web::main]
async fn main() -> async_graphql::Result<()> {
    let config = Config::load()?;
    env_logger::init();

    let pool = db::db_connection(&config).await?;
    add_private_column_to_repos(&pool).await?;
    add_prefix_column_to_repos(&pool).await?;
    add_root_topic_path_column_to_repos(&pool).await?;
    add_prefix_columns_to_users(&pool).await?;

    log::info!("database migrated.");
    Ok(())
}
