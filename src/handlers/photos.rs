use actix_web::{delete, get, post, web};
use deadpool_postgres::Pool;

use crate::requests::get_photos_request::GetPhotosRequest;
use crate::responses::api_response::ApiResponse;
use crate::schemas;
use crate::schemas::photo::Photo;
use crate::schemas::photo_full::PhotoFull;
use crate::types::HandlerResult;

// ALL PHOTOS **************************************************************************************

#[get("/photos")]
pub async fn get_photos(
    info: web::Query<GetPhotosRequest>,
    pool: web::Data<Pool>,
) -> HandlerResult {
    let page = PhotoFull::get_page(info.into_inner(), &pool).await?;

    Ok(ApiResponse::success(page))
}

// SINGLE PHOTO ************************************************************************************

#[get("/photos/{photo_id}")]
pub async fn get_photo(info: web::Path<i32>, pool: web::Data<Pool>) -> HandlerResult {
    let photo_id: i32 = info.into_inner();

    let photo = PhotoFull::get_by_id(photo_id, &pool).await?;

    Ok(ApiResponse::success(photo))
}

// UPDATE PHOTO ************************************************************************************

// #[post("/photos/{photo_id}")]
// pub async fn update_photo(
//     _: web::Path<i32>,
//     info: web::Json<Photo>,
//     pool: web::Data<Pool>,
// ) -> HandlerResult {
//     println!("{:?}", &info);
//     let photo = Photo::update_photo(info.into_inner(), &pool).await?;
//
//     Ok(ApiResponse::success(photo))
// }

#[post("/photos/{photo_id}/viewed")]
pub async fn update_photo_last_viewed(
    info: web::Path<i32>,
    pool: web::Data<Pool>,
) -> HandlerResult {
    let photo = Photo::update_last_viewed(info.into_inner(), &pool).await?;

    Ok(ApiResponse::success(photo))
}

#[post("/photos/{photo_id}/rating/{rating}")]
pub async fn update_photo_rating(
    info: web::Path<(i32, i32)>,
    pool: web::Data<Pool>,
) -> HandlerResult {
    let (photo_id, rating) = info.into_inner();
    let mut photo = Photo::get_by_id(photo_id, &pool).await?;
    photo.rating = rating;

    let photo = Photo::update_photo(photo, &pool).await?;

    Ok(ApiResponse::success(photo))
}

// DELETE PHOTO ************************************************************************************

#[delete("/photos/{id}")]
pub async fn delete_photo(info: web::Path<i32>, pool: web::Data<Pool>) -> HandlerResult {
    let message = Photo::delete_photo(info.into_inner(), &pool).await?;

    Ok(ApiResponse::success(message))
}

// PHOTO ENTITIES **********************************************************************************

#[post("/photos/{photo_id}/entities/{entity_id}")]
pub async fn add_entity_to_photo(
    info: web::Path<(i32, i32)>,
    pool: web::Data<Pool>,
) -> HandlerResult {
    let (photo_id, entity_id) = info.into_inner();
    let message = Photo::add_entity_to_photo(photo_id, entity_id, &pool).await?;

    Ok(ApiResponse::success(message))
}

#[delete("/photos/{photo_id}/entities/{entity_id}")]
pub async fn remove_entity_from_photo(
    info: web::Path<(i32, i32)>,
    pool: web::Data<Pool>,
) -> HandlerResult {
    let (photo_id, entity_id) = info.into_inner();

    let message = Photo::remove_entity_from_photo(photo_id, entity_id, &pool).await?;

    Ok(ApiResponse::success(message))
}

// PHOTO TAGS **************************************************************************************

#[post("/photos/{photo_id}/tags/{tag_id}")]
pub async fn add_tag_to_photo(info: web::Path<(i32, i32)>, pool: web::Data<Pool>) -> HandlerResult {
    let (photo_id, tag_id) = info.into_inner();

    let message = Photo::add_tag_to_photo(photo_id, tag_id, &pool).await?;

    Ok(ApiResponse::success(message))
}

#[delete("/photos/{photo_id}/tags/{tag_id}")]
pub async fn remove_tag_from_photo(
    info: web::Path<(i32, i32)>,
    pool: web::Data<Pool>,
) -> HandlerResult {
    let (photo_id, tag_id) = info.into_inner();

    let message = Photo::remove_tag_from_photo(photo_id, tag_id, &pool).await?;

    Ok(ApiResponse::success(message))
}

// PHOTO WALLPAPERS ********************************************************************************

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewWallpaper {
    pub file_path: String,
}

#[post("/photos/{photo_id}/wallpaper/{wallpaper_size_id}")]
pub async fn add_wallpaper_to_photo(
    params: web::Path<(i32, i32)>,
    info: web::Json<NewWallpaper>,
    pool: web::Data<Pool>,
) -> HandlerResult {
    let (photo_id, wallpaper_size_id) = params.into_inner();

    let message = Photo::add_wallpaper_to_photo(
        photo_id,
        wallpaper_size_id,
        info.into_inner().file_path,
        &pool,
    )
    .await?;

    Ok(ApiResponse::success(message))
}

#[delete("/photos/{photo_id}/wallpaper/{wallpaper_size_id}")]
pub async fn remove_wallpaper_from_photo(
    params: web::Path<(i32, i32)>,
    pool: web::Data<Pool>,
) -> HandlerResult {
    let (photo_id, wallpaper_size_id) = params.into_inner();

    let message = Photo::remove_wallpaper_from_photo(photo_id, wallpaper_size_id, &pool).await?;

    Ok(ApiResponse::success(message))
}

// RESET RANDOM SEED *******************************************************************************

#[get("/resetseed")]
pub async fn reset_seed(pool: web::Data<Pool>) -> HandlerResult {
    schemas::reset_seed(&pool).await?;

    Ok(ApiResponse::success(
        "`photo_ordering` materialized view was refreshed successfully",
    ))
}
