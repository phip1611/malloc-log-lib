all: info lib-dev lib-prod testprogram

info:
	@echo "This builds the lib (dev), the lib (release/prod) and the testprogram 'mallocfreetest' to see if it works"
	@echo "==========="

lib-dev:
	cargo build

lib-prod:
	cargo build --release

testprogram:
	gcc -o mallocfreetest mallocfreetest.c