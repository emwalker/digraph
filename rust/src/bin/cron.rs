use digraph::config::Config;
use digraph::db;
use digraph::prelude::*;

#[actix_web::main]
async fn main() -> Result<()> {
    let config = Config::load()?;
    env_logger::init();
    let pool = db::db_connection(&config).await?;

    sqlx::query(
        r#"
        with

        topic_stats as (
            select count(*) count from topics
        ),

        link_stats as (
            select count(*) count from links
        ),

        user_stats as (
            select count(*) count from users
        ),

        active_user_stats as (
            select count(*) count
            from (
                select distinct user_id
                from user_links where created_at > now() - interval '7 days'
                group by user_id
                having count(link_id) > 5
            ) a
        )

        insert into daily_snapshot (topic_count, link_count, user_count, active_user_count)
            select sum(t.count), sum(l.count), sum(u.count), sum(au.count)
            from topic_stats t
            cross join link_stats l
            cross join user_stats u
            cross join active_user_stats au
        "#,
    )
    .execute(&pool)
    .await?;

    log::info!("daily snapshot taken");
    Ok(())
}
