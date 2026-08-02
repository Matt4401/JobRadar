.PHONY: help check fmt fmt-check clippy test test-all run-scraper run-web run clean

CARGO = cargo

.DEFAULT_GOAL := help # default make

help:
	@echo "Usage: make [cible] [PKG=nom_du_module]"
	@echo ""
	@echo "Possibilitys:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'

check: fmt-check clippy test-all

fmt:
	$(CARGO) fmt --all

fmt-check:
	$(CARGO) fmt --all -- --check

clippy:
	@if [ -n "$(PKG)" ]; then \
		echo "Clippy over the module : $(PKG)"; \
		$(CARGO) clippy --package $(PKG) --all-targets -- -D warnings; \
	else \
		echo "Clippy over all workspace"; \
		$(CARGO) clippy --workspace --all-targets -- -D warnings; \
	fi

test:
	@if [ -n "$(PKG)" ]; then \
		echo "Test all the tests of the module : $(PKG)"; \
		$(CARGO) test --package $(PKG); \
	else \
		echo "Test all the tests of the workspace"; \
		$(CARGO) test --workspace; \
	fi

test-all:
	$(CARGO) test --workspace

run-scraper:
	$(CARGO) run -p scraper

run-web:
	$(CARGO) run -p web

run:
	$(CARGO) run -p scraper & $(CARGO) run -p web

clean:
	$(CARGO) clean
