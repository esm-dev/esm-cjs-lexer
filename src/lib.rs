mod cjs;
mod error;
mod swc;
mod test;

use swc::SWC;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Options {
  node_env: Option<String>,
  call_mode: Option<bool>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Output {
  pub exports: Vec<String>,
  pub reexports: Vec<String>,
}

#[wasm_bindgen(js_name = "parse")]
pub fn parse(specifier: &str, code: &str, options: JsValue) -> Result<JsValue, JsValue> {
  let options: Options = serde_wasm_bindgen::from_value(options).unwrap_or(Options {
    node_env: None,
    call_mode: None,
  });
  let ret = match SWC::parse(specifier, code) {
    Ok(ret) => ret,
    Err(e) => {
      return Err(JsError::new(&e.to_string()).into());
    }
  };
  let node_env = if let Some(env) = options.node_env {
    env
  } else {
    "production".to_owned()
  };
  let call_mode = if let Some(ok) = options.call_mode { ok } else { false };
  let (exports, reexports) = ret.get_exports(&node_env, call_mode);
  Ok(serde_wasm_bindgen::to_value(&Output { exports, reexports }).unwrap())
}
