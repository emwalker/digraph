use async_graphql::Result;
use sqlx::types::Uuid;
use std::collections::HashMap;
use std::result;

pub fn uuids(ids: &[String]) -> Vec<Uuid> {
    ids.iter()
        .map(|u| Uuid::parse_str(u).unwrap_or(Uuid::nil()))
        .collect::<Vec<Uuid>>()
}

pub trait BatchKey {
    fn batch_key(&self) -> String;
}

#[derive(Debug, Clone)]
pub enum BatchFnLoadError {
    NotFound,
    DBError(String),
}

pub type Value<T> = result::Result<T, BatchFnLoadError>;

fn errors<V>(e: sqlx::Error, ids: &[String]) -> HashMap<String, Value<V>> {
    ids.iter()
        .map(|id| ((*id).clone(), Err(BatchFnLoadError::DBError(e.to_string()))))
        .collect()
}

pub fn collect<V: BatchKey>(
    ids: &[String],
    rows: Result<Vec<V>, sqlx::Error>,
) -> HashMap<String, Value<V>> {
    match rows {
        Ok(rows) => {
            let lookup = rows
                .into_iter()
                .map(|row| (row.batch_key(), Ok(row)))
                .collect();

            ids.iter()
                .fold(lookup, |mut map: HashMap<String, Value<V>>, id: &String| {
                    map.entry((*id).to_string())
                        .or_insert(Err(BatchFnLoadError::NotFound));
                    map
                })
        }
        Err(e) => errors(e, ids),
    }
}

pub fn collect_relations<V: BatchKey + Clone>(
    ids: &[String],
    rows: Result<Vec<V>, sqlx::Error>,
) -> HashMap<String, Value<Vec<V>>> {
    match rows {
        Ok(rows) => {
            let mut groups = HashMap::<String, Vec<V>>::new();
            for row in rows.iter() {
                let key = row.batch_key();
                let v = groups.entry(key).or_insert(vec![]);
                v.push((*row).to_owned());
            }

            let mut lookup = HashMap::<String, Value<Vec<V>>>::new();

            lookup = groups.iter().fold(lookup, |mut map, (parent_id, rows)| {
                map.insert((*parent_id).to_owned(), Ok((*rows).to_owned()));
                map
            });

            ids.iter()
                .fold(lookup, |mut map: HashMap<String, Value<Vec<V>>>, id| {
                    map.entry(id.to_string())
                        .or_insert(Err(BatchFnLoadError::NotFound));
                    map
                })
        }
        Err(e) => errors(e, ids),
    }
}

pub fn unload<F, R, V>(result: Result<R, BatchFnLoadError>, ok: F) -> Result<Option<V>>
where
    F: Fn(R) -> V,
{
    match result {
        Ok(row) => Ok(Some(ok(row))),
        Err(err) => match err {
            BatchFnLoadError::NotFound => Ok(None),
            BatchFnLoadError::DBError(e) => {
                log::error!("there was a problem: {}", e);
                Ok(None)
            }
        },
    }
}
