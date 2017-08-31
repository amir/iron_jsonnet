extern crate libc;
extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use router::Router;
use iron::error::IronError;

use std::io::Read;
use libc::{c_char, c_int, c_uint, size_t};
use std::ffi::{CStr, CString};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JsonnetVm {
    _unused: [u8; 0],
}

#[link(name = "jsonnet")]
extern "C" {
    fn jsonnet_make() -> *mut JsonnetVm;
    fn jsonnet_destroy(vm: *mut JsonnetVm);
    fn jsonnet_max_stack(vm: *mut JsonnetVm, v: c_uint);
    fn jsonnet_max_trace(vm: *mut JsonnetVm, v: c_uint);
    fn jsonnet_gc_min_objects(vm: *mut JsonnetVm, v: c_uint);
    fn jsonnet_gc_growth_trigger(vm: *mut JsonnetVm, v: f64);
    fn jsonnet_jpath_add(vm: *mut JsonnetVm, v: *const c_char);
    fn jsonnet_realloc(vm: *mut JsonnetVm, buf: *mut c_char, sz: size_t);
    fn jsonnet_evaluate_snippet(
        vm: *mut JsonnetVm,
        filename: *const c_char,
        snippet: *const c_char,
        error: *mut c_int,
    ) -> *mut c_char;
}

fn main() {
    fn evaluate_snippet(request: &mut Request) -> IronResult<Response> {
        let mut payload = String::new();
        request.body.read_to_string(&mut payload).unwrap();
        let vm = unsafe {
            let vm = jsonnet_make();
            jsonnet_max_stack(vm, 500);
            jsonnet_gc_min_objects(vm, 1000);
            jsonnet_max_trace(vm, 20);
            jsonnet_gc_growth_trigger(vm, 2.0);
            jsonnet_jpath_add(vm, (*CString::new("./jpath").unwrap()).as_ptr());
            vm
        };
        let ev = unsafe {
            let filename = CString::new("").unwrap();
            let body = CString::new(payload).unwrap();
            let mut err: c_int = 0;
            let out =
                jsonnet_evaluate_snippet(vm, (*filename).as_ptr(), (*body).as_ptr(), &mut err);
            let res = CStr::from_ptr(out);
            jsonnet_realloc(vm, out, 0);
            jsonnet_destroy(vm);
            res
        };
        match ev.to_str() {
            Ok(v) => Ok(Response::with((status::Ok, v))),
            Err(err) => Err(IronError::new(err, status::InternalServerError)),
        }
    }

    let mut router = Router::new();
    router.post("/evaluate", evaluate_snippet, "evaluate");
    Iron::new(router).http("0.0.0.0:3000").unwrap();
}
