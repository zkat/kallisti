use std::collections::HashMap;

use kallisti_error::KallistiError;
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "invoke", js_namespace = ["window", "__TAURI__"], catch)]
    async fn _invoke(cmd: &str, arg: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = "convertFileSrc", js_namespace = ["window", "__TAURI__"])]
    async fn _convert_file_src(src: &str, protocol: Option<&str>) -> JsValue;
}

pub async fn invoke<T: Serialize>(
    cmd: impl AsRef<str>,
    arg: Option<HashMap<String, T>>,
) -> Result<JsValue, KallistiError> {
    _invoke(cmd.as_ref(), JsValue::from_serde(&arg.unwrap_or_default()).expect("Failed to convert arg to JsValue."))
        .await
        .map_err(|e| JsValue::into_serde(&e).expect("Failed to convert error."))
}
