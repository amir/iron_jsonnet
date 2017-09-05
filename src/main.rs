extern crate libc;
extern crate iron;
extern crate router;
extern crate yaml_rust;

use iron::prelude::*;
use iron::{Chain, status};
use router::Router;
use iron::error::IronError;
use yaml_rust::{YamlLoader, yaml};

use std::io::Read;
use std::string::String;
use std::ffi::{CStr, CString};
use std::collections::HashSet;
use libc::{c_char, c_int, c_uint, size_t};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JsonnetVm {
    _unused: [u8; 0],
}

#[link(name = "jsonnet")]
extern "C" {
    fn jsonnet_make() -> *mut JsonnetVm;
    fn jsonnet_version() -> *const c_char;
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

fn version_header(_: &mut Request, mut resp: Response) -> IronResult<Response> {
    let jv = {
        let v = unsafe { CStr::from_ptr(jsonnet_version()) };
        String::from(format!("Jsonnet/{}", v.to_str().ok().unwrap()))
    };
    resp.headers.set_raw("Server", vec![jv.into_bytes()]);

    Ok(resp)
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

    fn references(doc: &yaml::Yaml) -> Vec<String> {
        fn go(d: &yaml::Yaml, vec: &mut Vec<String>) {
            match *d {
                yaml::Yaml::Array(ref a) => {
                    for x in a {
                        go(x, vec)
                    }
                }
                yaml::Yaml::Hash(ref h) => {
                    for (_, v) in h {
                        go(v, vec)
                    }
                }
                yaml::Yaml::String(ref s) => vec.push((*s).clone()),
                ref otherwise => print!("Otherwise: {:?}", otherwise),
            }
        }

        let mut v: Vec<String> = Vec::new();
        go(doc, &mut v);

        v
    }

    fn root(x: &str) -> Option<String> {
        let vs: Vec<String> = x.split('.').map(str::to_owned).collect();
        if vs.len() > 1 {
            vs.get(0).map(|x| x.clone())
        } else {
            None
        }
    }

    fn parse_yaml(request: &mut Request) -> IronResult<Response> {
        use std::iter::FromIterator;

        let mut payload = String::new();
        request.body.read_to_string(&mut payload).unwrap();

        let docs = YamlLoader::load_from_str(&payload).unwrap();

        let libs: Vec<String> = docs.iter()
            .flat_map(references)
            .flat_map(|x| root(&x))
            .collect();
        let libs_set: HashSet<String> = HashSet::from_iter(libs);

        Ok(Response::with((status::Ok, format!("{:?}", libs_set))))
    }

    let mut router = Router::new();
    router.post("/evaluate", evaluate_snippet, "evaluate");
    router.post("/parse_yaml", parse_yaml, "parse_yaml");

    let mut chain = Chain::new(router);
    chain.link_after(version_header);

    Iron::new(chain).http(format!("0.0.0.0:{}", 3000)).unwrap();
}
