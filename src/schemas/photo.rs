use serde::{Serialize, Deserialize};
use chrono::{NaiveDateTime, Utc};
use tokio_postgres::Row;
use std::fs;
use deadpool_postgres::{PoolError, Pool};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Photo {
    pub id: i32,
    pub file_path: String,
    pub file_name: String,
    pub file_hash: String,
    pub rating: i32,
    pub date_created: NaiveDateTime,
    pub date_updated: NaiveDateTime,
    pub original_width: i32,
    pub original_height: i32,
    pub rotation: i32,
    pub ineligible_for_wallpaper: bool,
    pub anonymous_entities: bool,
}

impl Photo {
    pub fn from_row(row: Row) -> Self {
        Photo {
            id: row.get(0),
            file_path: row.get(1),
            file_name: row.get(2),
            file_hash: row.get(3),
            rating: row.get(4),
            date_created: row.get(5),
            date_updated: row.get(6),
            original_width: row.get(7),
            original_height: row.get(8),
            rotation: row.get(9),
            ineligible_for_wallpaper: row.get(10),
            anonymous_entities: row.get(11),
        }
    }

    pub async fn update_photo(updated_photo: Photo, pool: &Pool) -> Result<Self, PoolError> {
        let mut updated = updated_photo.clone();
        updated.date_updated = Utc::now().naive_utc();

        let client = pool.get().await?;
        let stmt = client
            .prepare(
                "UPDATE photos
                                    SET file_path = $1,
                                        file_name = $2,
                                        file_hash = $3,
                                        rating = $4,
                                        date_created = $5,
                                        date_updated = $6,
                                        original_width = $7,
                                        original_height = $8,
                                        rotation = $9,
                                        ineligible_for_wallpaper = $10,
                                        anonymous_entities = $11
                                    WHERE id = $12",
            )
            .await?;
        let _result = client
            .execute(
                &stmt,
                &[
                    &updated.file_path,
                    &updated.file_name,
                    &updated.file_hash,
                    &updated.rating,
                    &updated.date_created,
                    &updated.date_updated,
                    &updated.original_height,
                    &updated.original_width,
                    &updated.rotation,
                    &updated.ineligible_for_wallpaper,
                    &updated.anonymous_entities,
                    &updated.id,
                ],
            )
            .await?;

        let result = Photo::get_photo_by_id(updated.id as i64, pool).await?;

        Ok(result)
    }

    pub async fn get_photo_by_id(photo_id: i64, pool: &Pool) -> Result<Self, PoolError> {
        let client = pool.get().await?;
        let stmt = client.prepare("SELECT * FROM photos WHERE id = $1").await?;
        let result = client.query_one(&stmt, &[&photo_id]).await?;

        let photo = Photo::from_row(result);

        Ok(photo)
    }

    pub async fn get_photo_by_name(name: &str, hash: &str, pool: &Pool) -> Result<Self, PoolError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare("SELECT * FROM photos WHERE file_name = $1 AND file_path = $2")
            .await?;
        let result = client.query_one(&stmt, &[&name, &hash]).await?;

        let photo = Photo::from_row(result);

        Ok(photo)
    }

    pub async fn delete_photo(photo_id: i64, pool: &Pool) -> Result<String, PoolError> {
        let photo = Photo::get_photo_by_id(photo_id, &pool).await?;

        // attempt to delete photo
        fs::remove_file(&photo.file_path).expect("Could not delete file");

        let client = pool.get().await?;
        let stmt = client.prepare("DELETE FROM photos WHERE id = $1").await?;
        let _result = client.execute(&stmt, &[&photo_id]).await?;

        Ok("File deleted successfully!".to_string())
    }
}
