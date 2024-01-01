

all:	clean build verify

env:
	@echo
	@echo   '    source "$$HOME/.cargo/env"'
	@echo   '    source "$$HOME/export-esp.sh"'
	@echo   '    export PS1=" cargo $$ "'
	@echo

clean:
	@cargo clean

verify:
	@echo
	@date
	@echo
	@file ./target/xtensa-esp32-none-elf/debug/trial_ka_rr
	@echo

build:
	@rm -rf ./target/xtensa-esp32-none-elf/debug/trial_ka_rr
	@cargo build --target xtensa-esp32-none-elf

