use gloo::{
    worker::{
        Spawnable,
        oneshot::oneshot
    },
    net::http::Request,
    
};
use gloo::net::http::Request;
use gloo::utils::format::JsValueSerdeExt;

use wasm_bindgen::prelude::*;
use web_sys::js_sys;

#[oneshot]
pub async fn FetchData(url: String) -> Result<JsValue, wasm_bindgen::JsError> {

    let response = Request::get(&url)
        .send()
        .await
        .map_err(|err| JsValue::from_str(&format!("Network error: {}", err)))?;

    let data = response
        .text()
        .await
        .map_err(|err| JsValue::from_str(&format!("Failed to read response text: {}", err)))?;
    let json: serde_json::Value = serde_json::from_str(&data)
        .map_err(|err| JsValue::from_str(&format!("Failed to parse JSON: {}", err)))?;
    
    Ok(JsValue::from_serde(&json).map_err(|err| JsValue::from_str(&format!("Failed to convert to JsValue: {}", err)))?)
}

#[oneshot]
pub async fn FetchBinary(url: String) -> Result<Vec<u8>, wasm_bindgen::JsError> {
    let response = Request::get(&url)
        .send()
        .await
        .map_err(|err| JsValue::from_str(&format!("Network error: {}", err)))?;

    let bytes = response
        .binary()
        .await
        .map_err(|err| JsValue::from_str(&format!("Failed to read binary data: {}", err)))?;
    
    Ok(bytes)
}

#[wasm_bindgen]
pub fn fetch_data(url: String) -> js_sys::Promise {
    let mut worker = FetchData::spawner().spawn(
        "./some_api_endpoint"
    );
    js_sys::Promise::from(worker.run("/test".to_string()))
}

#[wasm_bindgen]
pub fn secret_fetcher() -> js_sys::Promise {
    let mut worker = FetchBinary::spawner().spawn(
        "./some_binary_endpoint"
    );
    js_sys::Promise::from(worker.run("/test".to_string()))
}