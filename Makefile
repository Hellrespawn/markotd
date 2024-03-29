PROGRAM := markotd
BIN_FOLDER := ${HOME}/.bin
BUILD_TARGET := target/release/${PROGRAM}

default:
	@echo "Usage:" 1>&2
	@echo "make install         # Build ${PROGRAM} and install to ${BIN_FOLDER}" 1>&2
	@echo "make uninstall       # Uninstall ${PROGRAM}" 1>&2
	@echo "make clean           # Clean build artifacts" 1>&2

install:
	@cargo build --release

	@mkdir -p ${BIN_FOLDER}

	@cp -p ${BUILD_TARGET} ${BIN_FOLDER}/${PROGRAM}

	@echo "Installed markotd to ${BIN_FOLDER}"

uninstall:
	@-rm ${BIN_FOLDER}/${PROGRAM} 2>/dev/null; true
	@-rmdir -p ${BIN_FOLDER} 2>/dev/null; true

	@echo "Removed markotd from ~/.bin"

clean:
	@cargo clean
