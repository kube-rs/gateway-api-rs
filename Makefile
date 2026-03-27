GATEWAY_API_VERSION ?= v1.4.1

.PHONY: all
all: build generate

.PHONY: clean
clean:
	cargo clean

.PHONY: build
build:
	cargo build

.PHONY: generate
generate:
	cargo xtask generate $(GATEWAY_API_VERSION)

.PHONY: test.all
test.all: test.unit test.integration

.PHONY: test.unit
test.unit:
	cargo test -vv -- --nocapture

.PHONY: test.integration
test.integration:
	cargo test -vv -- --nocapture --ignored
