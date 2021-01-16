#! /bin/bash

(echo Cargo.toml ; find src -name '*.rs') | entr -c wasm-pack build
