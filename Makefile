all: compiler

compiler:
	cargo build
	cp src/scripts/latc_llvm.sh latc_llvm
