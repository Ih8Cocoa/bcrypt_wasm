extern crate bcrypt;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use bcrypt::{hash, verify, BcryptError, BcryptResult};

pub type JsonResult<T> = Result<T, JsValue>;

#[wasm_bindgen]
pub fn hash_pw(password: &str, cost: u32) -> JsonResult<String> {
    let result = hash(password, cost);
    to_json_result(result)
}

#[wasm_bindgen]
pub fn check_pw(password: &str, hash: &str) -> JsonResult<bool> {
    let result = verify(password, hash);
    to_json_result(result)
}

fn to_json_result<T>(input: BcryptResult<T>) -> JsonResult<T> {
    match input {
        Ok(input) => Ok(input),
        Err(e) => {
            let error = cast_bc_error_to_str(e);
            Err(JsValue::from_str(error.as_str()))
        }
    }
}

fn cast_bc_error_to_str(e: BcryptError) -> String {
    let err_msg = match e {
        BcryptError::Io(_) => "An IO-related error has occurred",
        BcryptError::InvalidCost(_) => "Invalid Cost",
        BcryptError::CostNotAllowed(_) => "Cost not allowed",
        BcryptError::InvalidPassword => "Invalid Password: contains NULL byte",
        BcryptError::InvalidPrefix(_) => "Invalid Prefix",
        BcryptError::InvalidHash(_) => "Invalid hash",
        BcryptError::InvalidBase64(_, _) => "Invalid base64 char",
        BcryptError::Rand(_) => "An RNG-related error has occurred",
    };
    String::from(err_msg)
}