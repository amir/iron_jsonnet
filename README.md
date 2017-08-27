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
$ cat /tmp/example
local k = import "k8s.libsonnet";

// Specify the import objects that we need
local container = k.extensions.v1beta1.deployment.mixin.spec.template.spec.containersType;
local containerPort = container.portsType;

local targetPort = 80;
local podLabels = {app: "nginx"};

local nginxContainer =
  container.new("nginx", "nginx:1.7.9") +
  container.ports(containerPort.containerPort(targetPort));

nginxContainer


$ curl --data-binary "@/tmp/example" -X POST http://localhost:3000
{
   "image": "nginx:1.7.9",
   "name": "nginx",
   "ports": [
      {
         "containerPort": 80
      }
   ]
}
```
