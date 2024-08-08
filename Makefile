.PHONY: help serve build

# full path for the Makefile
ROOT_DIR:=$(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))
BASE_DIR:=$(shell basename $(ROOT_DIR))

.DEFAULT: help

help:
	@echo ;
	@echo "make serve" ;
	@echo "    :::> Build and serve the mdbook - opens in the default browser." ;
	@echo ;
	@echo "make build" ;
	@echo "    :::> Build the mdbook." ;
	@echo ;

# serve the book using the custom preprocessor
serve:
	mdbook serve --open content ;

# build the book using the custom preprocessor
build:
	mdbook build content ;
