extern crate libc;
extern crate iron;

use iron::prelude::*;
use iron::status;
use iron::error::IronError;

use libc::c_char;
use std::ffi::CStr;

#[link(name = "jsonnet")]
extern "C" {
    pub fn jsonnet_version() -> *const c_char;
}

fn main() {
    fn version(_: &mut Request) -> IronResult<Response> {
        let x = unsafe { CStr::from_ptr(jsonnet_version()) };
        match x.to_str() {
            Ok(v) => Ok(Response::with((status::Ok, v))),
            Err(err) => Err(IronError::new(err, status::InternalServerError)),
        }
    }

    let _server = Iron::new(version).http("0.0.0.0:3000").unwrap();
}
