.PHONY: clean
clean:
	cargo clean

.PHONY: generate
generate:
	./update.sh

.PHONY: test.all
test.all: test.unit test.integration

.PHONY: test.unit
test.unit:
	cargo test -vv -- --nocapture

.PHONY: test.integration
test.integration:
	cargo test -vv -- --nocapture --ignored
