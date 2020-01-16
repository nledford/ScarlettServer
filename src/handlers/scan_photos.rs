use actix_web::{get, web, HttpResponse};
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};

use crate::files;
use crate::models::errors;
use crate::schemas::new_photo::NewPhoto;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanPhotosResult {
    pub new_photos_found: bool,
    pub new_photos: i32,
    pub updated_photos: i32,
    pub deleted_photos: i32,
}

impl Default for ScanPhotosResult {
    fn default() -> ScanPhotosResult {
        ScanPhotosResult {
            new_photos_found: false,
            new_photos: 0,
            updated_photos: 0,
            deleted_photos: 0,
        }
    }
}

impl ScanPhotosResult {
    pub fn new(new_photos: i32, updated_photos: i32, deleted_photos: i32) -> Self {
        ScanPhotosResult {
            new_photos_found: new_photos > 0,
            new_photos,
            updated_photos,
            deleted_photos,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScanPhotosRequest {
    pub folder: Option<String>,
}

impl ScanPhotosRequest {
    pub fn get_folder(&self) -> String {
        self.folder
            .to_owned()
            .unwrap_or_else(|| String::from(""))
            .replace('\"', "")
    }
}

#[get("/scan")]
pub async fn run_scan(
    info: web::Query<ScanPhotosRequest>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, errors::Error> {
    let folder = info.get_folder();
    let pool = pool.get_ref();

    let file_scan_result = if !folder.is_empty() {
        files::photos::scan_all_photos_from_dir(&folder, pool).await
    } else {
        files::photos::scan_all_photos(pool).await
    };

    if let Err(err) = file_scan_result {
        return Ok(HttpResponse::InternalServerError().json(err.to_string()));
    }
    let file_scan_result = file_scan_result.unwrap();

    let files = file_scan_result.new_photos;

    let new_photos = NewPhoto::bulk_insert(files, pool).await?;

    let result = ScanPhotosResult::new(
        new_photos as i32,
        file_scan_result.updated_photos_count,
        file_scan_result.deleted_photos_count,
    );

    Ok(HttpResponse::Ok().json(result))
}