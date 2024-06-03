use actix_web::{web, Error, HttpResponse, post};
use futures::StreamExt;
use std::io::Write;
use actix_multipart::Multipart;
use futures_util::TryStreamExt;
use log::debug;
use crate::AppState;
use crate::biz::courier::SadCourier;
use crate::infra::error::error::Kind::InfraError;
use crate::infra::error::error::ServiceError;

/// 上传头像处理函数
#[post("/image")]
pub async fn save_image(
    app_state: web::Data<AppState>,
    payload: Multipart) -> Result<HttpResponse, Error> {
    handle_file_upload(payload, &app_state.image_static_dir, "image").await?;

    Ok(HttpResponse::Ok().json(
        SadCourier::brief("Success to upload image")
    ))
}

#[post("/document")]
pub async fn save_document(
    app_state: web::Data<AppState>,
    payload: Multipart) -> Result<HttpResponse, Error> {
    handle_file_upload(payload, &app_state.document_static_dir, "document").await?;

    Ok(HttpResponse::Ok().json(
        SadCourier::brief("Success to upload document")
    ))
}


/// 通用文件上传处理函数
async fn handle_file_upload(
    mut payload: Multipart,
    upload_dir: &str,
    field_name: &str,
) -> Result<(), ServiceError> {
    while let Ok(Some(mut field)) = payload.try_next().await {
        debug!("field: {:?}", field);
        debug!("field name: {:?}", field.name());
        if field.name() == field_name {
            let content_disposition = field.content_disposition();
            let filename = content_disposition
                .get_filename()
                .ok_or_else(||
                    ServiceError::build()
                        .belong(InfraError)
                        .message("Failed to get file name from header of content disposition")
                        .done()
                )?;

            // let now = Utc::now().timestamp();
            // let unique_filename = format!("{}_{}_{}", user_id, now, sanitize_filename::sanitize(filename));

            let filepath = format!("{}/{}", upload_dir, sanitize_filename::sanitize(filename));

            // 创建文件并写入数据
            let mut f = web::block(|| std::fs::File::create(filepath)).await??;
            while let Some(chunk) = field.next().await {
                let data = chunk?;
                f = web::block(move || f.write_all(&data).map(|_| f)).await??;
            }
        }
    }
    Ok(())
}