
# makefile
# borrowed from https://github.com/piaoger/golagger/Makefile

OS := $(shell uname -s)
THIS_FOLDER := $(shell cd ${0%/*} && echo ${PWD})
TIMESTAMP := $(shell date "+%G%m%d%H%M%S")
TODAY := $(shell date "+%G%m%d")

export PATH := ${THIS_FOLDER}/bin:${HOME}/.cargo/bin:${PATH}

.PHONY: build test

all: bootstrap godeps-update build test

help:
	@echo "#-------------------------------------"
	@echo "# rust binding for tess2!"
	@echo "#-------------------------------------"
	@echo " build           - build the project"
	@echo " update-bindings - update bindings"
	@echo " clean           - cleanup and refresh"
	@echo " updatedeps      - update libtess2 dependencies"
	@echo " fmt             - format code"
	@echo " test            - run test"

build:
	@echo 'clean up and refresh ...'
	@cargo build --release

update-bindings:
	@cargo build --release  --features="update-bindings"

test:
	@echo "TODO: health checking ..."
	@cargo test

fmt:
	@echo 'golang code auto formatting ...'
	@cargo fmt

clean:
	@echo 'clean up and refresh ...'
	@cargo clean

updatedeps:
	@echo 'update dependencies ...'
	@rm -rf tess2-sys/native
	@cd tess2-sys && git clone https://github.com/memononen/libtess2.git native
	@# remove hidden .git stuffs, so that they can be put into git repo
	@echo 'deps are disconnectted from git ...'
	@bash -c "find native -name '.gitignore'   -and -type f|xargs rm -f"
	@bash -c "find native -name 'premake4.lua' -and -type f|xargs rm -f"
	@bash -c "find native -name '.git'         -and -type d|xargs rm -rf"
	@bash -c "find native -name 'Bin'          -and -type d|xargs rm -rf"
	@bash -c "find native -name 'Example'      -and -type d|xargs rm -rf"
	@bash -c "find native -name 'Contrib'      -and -type d|xargs rm -rf"

