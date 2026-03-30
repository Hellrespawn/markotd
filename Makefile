PROGRAM := markotd

default:
	@echo "Usage:" 1>&2
	@echo "make install         # Install ${PROGRAM} with cargo install" 1>&2
	@echo "make uninstall       # Uninstall ${PROGRAM}" 1>&2
	@echo "make clean           # Clean build artifacts" 1>&2

install:
	@cargo install --path .
	@echo "Installed ${PROGRAM} with cargo install"

uninstall:
	@cargo uninstall ${PROGRAM}
	@echo "Removed ${PROGRAM} with cargo uninstall"

clean:
	@cargo clean
