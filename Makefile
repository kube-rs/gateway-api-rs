.PHONY: generate
generate:
	./update.sh

.PHONY: test.all
test.all:
	cargo test -vv -- --nocapture --ignored
