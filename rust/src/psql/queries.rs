pub const LINK_FIELDS: &str = r#"
    concat('/', o.login, '/', l.id) path,
    l.title,
    l.url,
    l.repository_id,
    null reviewed_at,
    -- Guest user
    '11a13e26-ee64-4c31-8af1-d1e953899ee0' viewer_id,
    array_remove(array_agg(distinct concat('/', o.login, '/', parent_topics.parent_id)), null)
        parent_topic_paths
"#;

pub const LINK_JOINS: &str = r#"
    from links l
    join repositories r on r.id = l.repository_id
    join organization_members om on om.organization_id = r.organization_id
    join organizations o on o.id = l.organization_id
    left join link_topics parent_topics on l.id = parent_topics.child_id
"#;
