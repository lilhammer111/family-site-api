use actix_web::{web, Error, HttpResponse, post};
use futures::StreamExt;
use std::io::Write;
use actix_multipart::Multipart;
use futures_util::TryStreamExt;

/// 上传头像处理函数
#[post("/avatar")]
async fn save(mut payload: Multipart) -> Result<HttpResponse, Error> {
    while let Ok(Some(mut field)) = payload.try_next().await {
        if field.name() == "file" { // 确保字段名匹配
            let content_type = field.content_disposition();
            let filename = content_type.get_filename().unwrap();
            let filepath = format!("/home/lilhammer/Dev/Static/Avatar/{}", sanitize_filename::sanitize(filename));

            // 创建文件并写入数据
            let mut f = web::block(|| std::fs::File::create(filepath)).await??;
            while let Some(chunk) = field.next().await {
                let data = chunk?;
                f = web::block(move || f.write_all(&data).map(|_| f)).await??;
            }
        }
    }

    Ok(HttpResponse::Ok().body("File uploaded successfully"))
}