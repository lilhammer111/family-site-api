use actix_web::{web, Error, HttpResponse, post, HttpRequest};
use futures::StreamExt;
use std::io::Write;
use actix_multipart::Multipart;
use chrono::Utc;
use futures_util::TryStreamExt;
use log::debug;
use crate::AppState;
use crate::biz::courier::SadCourier;
use crate::biz::internal::extract_user_id;
use crate::infra::error::error::Kind::InfraError;
use crate::infra::error::error::ServiceError;

/// 上传头像处理函数
#[post("/image")]
async fn save(req: HttpRequest, app_state: web::Data<AppState>, mut payload: Multipart) -> Result<HttpResponse, Error> {
    let user_id = extract_user_id(req)?;

    while let Ok(Some(mut field)) = payload.try_next().await {
        debug!("field: {:?}", field);
        debug!("field name: {:?}", field.name());
        if field.name() == "image" {
            let content_disposition = field.content_disposition();
            let filename = content_disposition
                .get_filename()
                .ok_or(
                    ServiceError::build()
                        .belong(InfraError)
                        .message("Failed to get file name from header of content disposition")
                        .done()
                )?;

            // let now = Utc::now().timestamp();
            //
            // let unique_filename = format!("{}_{}_{}", user_id, now, sanitize_filename::sanitize(filename));

            let filepath = format!("{}/{}", app_state.path_to_static_dir, sanitize_filename::sanitize(filename));

            // 创建文件并写入数据
            let mut f = web::block(|| std::fs::File::create(filepath)).await??;
            while let Some(chunk) = field.next().await {
                let data = chunk?;
                f = web::block(move || f.write_all(&data).map(|_| f)).await??;
            }
        }
    }

    Ok(HttpResponse::Ok().json(
        SadCourier::brief("Success to upload image")
    ))
}

