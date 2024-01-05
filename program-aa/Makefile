

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
	@file ./target/xtensa-esp32-none-elf/debug/program-aa
	@echo

build:
	@rm -rf ./target/xtensa-esp32-none-elf/debug/program-aa
	@cargo build --target xtensa-esp32-none-elf

rvim:
	@echo
	@echo '    export PS1=" rvim $$ "'
	@echo

# @echo '    export PS1=" rvim $$ "'

# end.
