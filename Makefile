.PHONY: help serve

# full path for the Makefile
ROOT_DIR:=$(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))
BASE_DIR:=$(shell basename $(ROOT_DIR))

.DEFAULT: help

help:
	@echo ;
	@echo "make check" ;
	@echo "    :::> Cargo check workspace with all features activated." ;
	@echo ;
	@echo "make test" ;
	@echo "    :::> Run all tests in workspace with all features using 4 threads." ;
	@echo ;

# build the book using the backend
serve:
	mdbook serve --open content ;
