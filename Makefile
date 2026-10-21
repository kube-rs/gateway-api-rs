GATEWAY_API_VERSION ?= v1.5.1
KIND_CLUSTER_NAME ?= gateway-api-test
KIND_KUBECONFIG := /tmp/$(KIND_CLUSTER_NAME)-kubeconfig

CLIPPY_ALLOWS := -A clippy::doc_lazy_continuation \
	-A clippy::tabs_in_doc_comments \
	-A clippy::derivable_impls

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

.PHONY: fmt.check
fmt.check:
	cargo +nightly fmt -- --check

.PHONY: lint
lint: fmt.check
	cargo deny check
	cargo clippy --all-targets -- -D warnings $(CLIPPY_ALLOWS)
	cargo clippy --all-targets --all-features -- -D warnings $(CLIPPY_ALLOWS)

.PHONY: package
package:
	cargo package -p gateway-api --features k8s-openapi/earliest

.PHONY: generate
generate:
	cargo xtask generate $(GATEWAY_API_VERSION)

define run_integration_tests
	@$(MAKE) --no-print-directory kind.start
	@trap '$(MAKE) --no-print-directory kind.stop' EXIT; \
	KUBECONFIG=$(KIND_KUBECONFIG) cargo test -vv $(1) -- --nocapture --ignored
endef

.PHONY: test.all
test.all: test.unit test.unit.experimental test.integration test.integration.experimental

.PHONY: test.unit
test.unit:
	cargo test -vv -- --nocapture

.PHONY: test.unit.experimental
test.unit.experimental:
	cargo test -vv --features experimental -- --nocapture

.PHONY: test.integration
test.integration:
	$(call run_integration_tests)

.PHONY: test.integration.experimental
test.integration.experimental:
	$(call run_integration_tests,--features experimental)

.PHONY: kind.start
kind.start:
	@kind get clusters 2>/dev/null | grep -q '^$(KIND_CLUSTER_NAME)$$' || \
		kind create cluster --name $(KIND_CLUSTER_NAME)
	@kind get kubeconfig --name $(KIND_CLUSTER_NAME) > $(KIND_KUBECONFIG)

.PHONY: kind.stop
kind.stop:
	@kind delete cluster --name $(KIND_CLUSTER_NAME) 2>/dev/null || true
	@rm -f $(KIND_KUBECONFIG)
