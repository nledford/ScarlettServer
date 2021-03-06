use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;

use crate::types::{DbMessageResult, DbSingleResult, DbVecResult};

#[derive(Serialize, Deserialize, Debug, Clone, PostgresMapper)]
#[pg_mapper(table = "tags")]
pub struct Tag {
    pub id: i32,
    pub tag_name: String,
}

impl Tag {
    pub async fn get_all(pool: &Pool) -> DbVecResult<Self> {
        let client = pool.get().await?;
        let stmt = client.prepare("SELECT * FROM tags").await?;
        let results = client.query(&stmt, &[]).await?;
        let tags: Vec<Tag> = results
            .into_iter()
            .map(|result| Tag::from_row(result).unwrap())
            .collect();

        Ok(tags)
    }

    pub async fn get_by_id(id: i32, pool: &Pool) -> DbSingleResult<Self> {
        let client = pool.get().await?;
        let stmt = client.prepare("SELECT * FROM tags WHERE id = $1").await?;
        let result = client.query_one(&stmt, &[&id]).await?;
        let tag = Tag::from_row(result).unwrap();

        Ok(tag)
    }

    pub async fn create(tag_name: &str, pool: &Pool) -> DbSingleResult<Self> {
        let client = pool.get().await?;
        let stmt = client
            .prepare("INSERT INTO tags (tag_name) VALUES($1)")
            .await?;
        let _ = client.execute(&stmt, &[&tag_name]).await?;

        let tag = Tag::get_by_name(tag_name, pool).await?;

        Ok(tag)
    }

    pub async fn get_by_name(tag_name: &str, pool: &Pool) -> DbSingleResult<Self> {
        let client = pool.get().await?;
        let stmt = client
            .prepare("SELECT * FROM tags WHERE tag_name = $1")
            .await?;
        let result = client.query_one(&stmt, &[&tag_name]).await?;
        let tag = Tag::from_row(result).unwrap();

        Ok(tag)
    }

    pub async fn update(tag: Tag, pool: &Pool) -> DbSingleResult<Self> {
        let client = pool.get().await?;
        let stmt = client
            .prepare("UPDATE tags SET tag_name = $1 WHERE id = $2")
            .await?;
        let _ = client.execute(&stmt, &[&tag.tag_name, &tag.id]).await?;

        let updated_tag = Tag::get_by_id(tag.id, pool).await?;

        Ok(updated_tag)
    }

    pub async fn delete(id: i32, pool: &Pool) -> DbMessageResult {
        let tag_to_delete = Tag::get_by_id(id, pool).await?;

        let client = pool.get().await?;
        let stmt = client.prepare("DELETE FROM tags WHERE id = $1").await?;
        let _ = client.execute(&stmt, &[&tag_to_delete.id]);

        Ok("Tag deleted successfully".to_string())
    }

    pub async fn perform_search(q: String, pool: &Pool) -> DbVecResult<Self> {
        let client = pool.get().await?;
        let stmt = client
            .prepare(
                "select * \
                 from tags \
                 order by similarity(tag_name, $1) desc \
                 limit 5",
            )
            .await?;
        let result = client.query(&stmt, &[&q]).await?;

        let search_results: Vec<Tag> = result
            .into_iter()
            .map(|result| Tag::from_row(result).unwrap())
            .collect();

        Ok(search_results)
    }
}
