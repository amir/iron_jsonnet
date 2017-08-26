```
$ cat /tmp/example.jsonnet
local Kube = import "kube.libsonnet";

{

    "nginx-rc.yaml": Kube.v1.ReplicationController("nginx") {
    	spec: {
		replicas: 1
	},
    },
    "nginx-svc.yaml": Kube.v1.Service("nginx") {
    	spec: {
		selector: {
			name: "nginx",
		},
	},
    }
}

$ curl --data-binary "@/tmp/example.jsonnet" -X POST http://localhost:3000
{
   "nginx-rc.yaml": {
      "apiVersion": "v1",
      "kind": "ReplicationController",
      "metadata": {
         "labels": {
            "name": "nginx"
         },
         "name": "nginx"
      },
      "spec": {
         "replicas": 1
      }
   },
   "nginx-svc.yaml": {
      "apiVersion": "v1",
      "kind": "Service",
      "metadata": {
         "labels": {
            "name": "nginx"
         },
         "name": "nginx"
      },
      "spec": {
         "selector": {
            "name": "nginx"
         }
      }
   }
}
```
