# Ironned Jsonnet
Creates a Jsonnet VM per HTTP request and evaluates its body payload.

## Requirements
* [Jsonnet](http://jsonnet.org/)
* [ksonnet-gen](https://github.com/ksonnet/ksonnet-lib/tree/master/ksonnet-gen)

This application uses Rust FFI to call into `Jsonnet` library, and `ksonnet-gen` to generate a `Jsonnet` library from [Kubernetes OpenAPI definition](https://kubernetes.io/docs/concepts/overview/kubernetes-api/).

Compiling and linking requires `libjsonnet.so`, and also `ksonnet-gen` requires the Go compiler to build. There's a `Makefile` which takes care of all of these dependencies and will run the web service (all dependencies are installed in `.deps/` so they don't interfere with your Go or Jsonnet installation).

`Jsonnet` VM will look for libraries in [jpath](jpath) and `k8s.libsonnet` is put there as well, upon generation.

As mentioned earlier the application is written in Rust so you'll require Rust in order for the `make run` to work. If you don't have Rust installed, installing it using [rustup](https://rustup.rs/) should be easy.

Once you have Rust and Cargo:
```
$ make run
```
And after a few minutes you should have a running web service listening on port 3000.

```
$ cat /tmp/example.yaml
metadata:
  labels:
  - acme.version('1.1')
spec:
  containers:
   - acme.webserver('lb')

$ curl --data-binary "@/tmp/example" -X POST http://localhost:3000/evaluate
metadata:
  labels:
    -
      version: "1.1"
spec:
  containers:
    -
      image: gcr.io/google_containers/nginx
      name: lb
```
