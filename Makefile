GATEWAY_API_VERSION ?= v1.5.1
KIND_CLUSTER_NAME ?= gateway-api-test
KIND_KUBECONFIG := /tmp/$(KIND_CLUSTER_NAME)-kubeconfig

.PHONY: all
all: build generate

.PHONY: clean
clean:
	cargo clean

.PHONY: build
build:
	cargo build

.PHONY: fmt
fmt:
	cargo +nightly fmt

.PHONY: lint
lint: fmt
	cargo deny check
	cargo clippy --all-targets --all-features -- -D warnings \
		-A clippy::doc_lazy_continuation \
		-A clippy::tabs_in_doc_comments \
		-A clippy::derivable_impls

.PHONY: generate
generate:
	cargo xtask generate $(GATEWAY_API_VERSION)

.PHONY: test.all
test.all: test.unit test.integration

.PHONY: test.unit
test.unit:
	cargo test -vv -- --nocapture

.PHONY: test.integration
test.integration: kind.start
	@trap '$(MAKE) kind.stop' EXIT; \
	KUBECONFIG=$(KIND_KUBECONFIG) cargo test -vv -- --nocapture --ignored

.PHONY: kind.start
kind.start:
	@kind get clusters 2>/dev/null | grep -q '^$(KIND_CLUSTER_NAME)$$' || \
		kind create cluster --name $(KIND_CLUSTER_NAME)
	@kind get kubeconfig --name $(KIND_CLUSTER_NAME) > $(KIND_KUBECONFIG)

.PHONY: kind.stop
kind.stop:
	@kind delete cluster --name $(KIND_CLUSTER_NAME) 2>/dev/null || true
	@rm -f $(KIND_KUBECONFIG)
