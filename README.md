```
$ make run
```

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
