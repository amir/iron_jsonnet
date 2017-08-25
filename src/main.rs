extern crate libc;
extern crate iron;

use iron::prelude::*;
use iron::status;
use iron::error::IronError;

use std::io::Read;
use libc::{c_char, c_int, c_uint};
use std::ffi::{CStr, CString};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JsonnetVm {
    _unused: [u8; 0],
}

#[link(name = "jsonnet")]
extern "C" {
    pub fn jsonnet_make() -> *mut JsonnetVm;
    pub fn jsonnet_version() -> *const c_char;
    pub fn jsonnet_max_stack(vm: *mut JsonnetVm, v: c_uint);
    pub fn jsonnet_max_trace(vm: *mut JsonnetVm, v: c_uint);
    pub fn jsonnet_gc_min_objects(vm: *mut JsonnetVm, v: c_uint);
    pub fn jsonnet_gc_growth_trigger(vm: *mut JsonnetVm, v: f64);
    pub fn jsonnet_evaluate_snippet(vm: *mut JsonnetVm, filename: *const c_char, snippet: *const c_char, error: *mut c_int) -> *mut c_char;
}

fn main() {
    fn version(request: &mut Request) -> IronResult<Response> {
        let mut payload = String::new();
        request.body.read_to_string(&mut payload).unwrap();
        let vm = unsafe {
            let vm = jsonnet_make();
            jsonnet_max_stack(vm, 500);
            jsonnet_gc_min_objects(vm, 1000);
            jsonnet_max_trace(vm, 20);
            jsonnet_gc_growth_trigger(vm, 2.0);
            vm
        };
        let ev = unsafe {
            let filename = CString::new("").unwrap();
            let body = CString::new(payload).unwrap();
            let mut err: c_int = 0;
            let e = jsonnet_evaluate_snippet(vm, (*filename).as_ptr(), (*body).as_ptr(), &mut err);
            CStr::from_ptr(e)
        };
        match ev.to_str() {
            Ok(v) => Ok(Response::with((status::Ok, v))),
            Err(err) => Err(IronError::new(err, status::InternalServerError)),
        }
    }

    let _server = Iron::new(version).http("0.0.0.0:3000").unwrap();
}
