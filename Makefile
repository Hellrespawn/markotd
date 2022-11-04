PROGRAM := markotd
PARTS_FOLDER := ${HOME}/.${PROGRAM}.d
BIN_FOLDER := ${HOME}/.bin

default:
	@echo "Usage:" 1>&2
	@echo "make install    # Install ${PROGRAM} to ${BIN_FOLDER} and ${PARTS_FOLDER}" 1>&2
	@echo "make uninstall  # Uninstall ${PROGRAM}" 1>&2

install:
	@mkdir -p ${BIN_FOLDER}
	@cp -p markotd.sh ${BIN_FOLDER}/${PROGRAM}

	@echo "Installed markotd to ${BIN_FOLDER}"

	@-rm -r ${PARTS_FOLDER} 2>/dev/null; true
	@cp -r ./markotd.d ${PARTS_FOLDER}

	@echo "Installed parts to ${PARTS_FOLDER}"

uninstall:
	@-rm ${BIN_FOLDER}/${PROGRAM} 2>/dev/null; true
	@-rmdir -p ${BIN_FOLDER} 2>/dev/null; true

	@echo "Removed markotd from ~/.bin"

	@-rm -r ${PARTS_FOLDER} 2>/dev/null; true
	@-rmdir -p ${PARTS_FOLDER} 2>/dev/null; true

	@echo "Removed parts from ${PARTS_FOLDER}"
