use raw::{Env, Local};

use nodejs_sys as napi;

pub unsafe extern "C" fn throw(env: Env, error: Local) {
    let status = napi::napi_throw(env, error);
    assert_eq!(status, napi::napi_status::napi_ok);
}

pub unsafe extern "C" fn new_error(out: &mut Local, env: Env, code: Local, msg: Local) {
    let status = napi::napi_create_error(env, code, msg, out);
    assert_eq!(status, napi::napi_status::napi_ok);
}

pub unsafe extern "C" fn new_type_error(out: &mut Local, env: Env, code: Local, msg: Local) {
    let status = napi::napi_create_type_error(env, code, msg, out);
    assert_eq!(status, napi::napi_status::napi_ok);
}

pub unsafe extern "C" fn new_range_error(out: &mut Local, env: Env, code: Local, msg: Local) {
    let status = napi::napi_create_range_error(env, code, msg, out);
    assert_eq!(status, napi::napi_status::napi_ok);
}
