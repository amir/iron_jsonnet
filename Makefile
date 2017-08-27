OS     := $(subst Darwin,darwin,$(subst Linux,linux,$(shell uname)))
ARCH   := $(shell uname -m)
GOVER  := go1.9
GOOS   := $(subst Darwin,darwin,$(subst Linux,linux,$(OS)))
GOARCH := $(subst x86_64,amd64,$(ARCH))
GOPKG  := $(subst darwin-amd64,darwin-amd64,$(GOVER).$(GOOS)-$(GOARCH).tar.gz)
GOROOT := $(CURDIR)/.deps/go
GOPATH := $(CURDIR)/.deps/gopath
GOCC   := $(GOROOT)/bin/go
PATH   := $(GOPATH)/bin:$(CURDIR)/bin:$(PATH)

GOENV := GOROOT=$(GOROOT)
GBBIN := $(GOPATH)/bin/gb
GO    := $(GOENV) $(GOCC)

JSONNET         := $(CURDIR)/.deps/jsonnet
LIBJSONNET      := $(CURDIR)/.deps/jsonnet/libjsonnet.so
JSONNETLIBSPATH := $(CURDIR)/jpath

KSONNETLIB := $(JSONNETLIBSPATH)/k8s.libsonnet
KSONNETGEN := $(GOPATH)/bin/ksonnet-gen

K8SVERSION     := v1.7.0
K8SOPENAPISPEC := $(GOPATH)/src/k8s.io/kubernetes/api/openapi-spec/swagger.json

.PHONY: run
run: $(LIBJSONNET) $(KSONNETLIB)
	LD_LIBRARY_PATH=$(JSONNET) cargo run

.PHONY: clean
clean:
	rm -rf .deps

$(JSONNET):
	mkdir -p .deps
	git clone https://github.com/google/jsonnet.git $(JSONNET)

$(LIBJSONNET): $(JSONNET)
	cd $(JSONNET) && make libjsonnet.so

$(GOCC): .deps/$(GOPKG)
	tar -C .deps -xzf .deps/$(GOPKG)
	touch $@

.deps/$(GOPKG):
	mkdir -p .deps
	curl -o .deps/$(GOPKG) https://storage.googleapis.com/golang/$(GOPKG)

$(KSONNETGEN): $(GOCC)
	GOPATH=$(GOPATH) GOROOT=$(GOROOT) $(GOCC) get github.com/ksonnet/ksonnet-lib/ksonnet-gen

$(K8SOPENAPISPEC): $(GOCC)
	GOPATH=$(GOPATH) GOROOT=$(GOROOT) $(GOCC) get k8s.io/kubernetes/pkg/api
	cd $(GOPATH)/src/k8s.io/kubernetes && git checkout $(K8SVERSION)

$(KSONNETLIB): $(KSONNETGEN) $(K8SOPENAPISPEC)
	$(KSONNETGEN) $(K8SOPENAPISPEC) $(JSONNETLIBSPATH)
	sed -i.back -e "s/local::/localPath::/g" $(KSONNETLIB)
	rm $(KSONNETLIB).back
