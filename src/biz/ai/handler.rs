use std::collections::HashMap;
use actix_web::{HttpResponse, post, Error, web};
use log::debug;
use serde::{Deserialize, Serialize};
use crate::AppState;
use crate::infra::error::error::ServiceError;


const KIMI_API_URL: &str = "https://api.moonshot.cn/v1/chat/completions";

#[derive(Debug, Deserialize, Serialize)]
struct AiReq {
    messages: Vec<HashMap<String, String>>,
    model: String,
    max_token: Option<i32>,
    temperature: Option<f32>,
    top_p: Option<f32>,
    n: Option<i32>,
    presence_penalty: Option<f32>,
    frequency_penalty: Option<f32>,
    stop: Option<Vec<String>>,
}

#[post("")]
pub async fn get_ai_response(app_state: web::Data<AppState>, req_json: web::Json<AiReq>) -> Result<HttpResponse, Error> {
    let req = req_json.into_inner();
    debug!("req: {:?}",req);

    let client = reqwest::Client::new();

    let ai_resp = client.post(KIMI_API_URL)
        .header(
            "Authorization",
            format!("Bearer {}", app_state.kimi_secret),
        )
        .json(&req)
        .send().await
        .map_err(|e|Into::<ServiceError>::into(e))?
        .text().await
        .map_err(|e|Into::<ServiceError>::into(e))?;

    debug!("{:?}", ai_resp);

    Ok(
        HttpResponse::Ok()
            .content_type("application/json")
            .body(ai_resp)
    )
}