.PHONY: dev
dev:
	cargo run

.PHONY: build
build:
	cargo build --release

.PHONY: test
test:
	cargo test --all

.PHONY: fmt
fmt:
	cargo fmt --all -- --check

.PHONY: check
check:
	cargo check --all

.PHONY: clippy
clippy:
	cargo clippy --all -- -D warnings

.PHONY: clean
clean:
	cargo clean

.PHONY: migpre
prepare:
	cargo sqlx prepare

.PHONY: migrun
migrun:
	sqlx migrate run

.PHONY: migrev
migrev:
	sqlx migrate revert

.PHONY: miginfo
miginfo:
	sqlx migrate info

.PHONY: migadd
migadd:
	sqlx migrate add -r $(name)
