use actix_web::{web, Error, HttpResponse, post};
use futures::StreamExt;
use std::io::Write;

/// 上传头像处理函数
#[post("/avatar")]
async fn upload(mut payload: web::Payload) -> Result<HttpResponse, Error> {
    let mut bytes = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let data = chunk?;
        bytes.extend_from_slice(&data);
    }

    // 假设 `bytes` 包含了整个图像文件数据
    let mut file = web::block(|| std::fs::File::create("uploaded_image.png")).await??;
    web::block(move || file.write_all(&bytes)).await??;

    Ok(HttpResponse::Ok().body("Image uploaded successfully"))
}