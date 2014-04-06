RUSTC ?= rustc
RUSTTEST ?= rustc --test
RUSTFLAGS ?= -O --out-dir build -L build

bin: build lib
	$(RUSTC) $(RUSTFLAGS) src/main.rs

lib: build
	$(RUSTC) $(RUSTFLAGS) src/lib.rs

build:
	mkdir -p build

test: build
	$(RUSTTEST) $(RUSTFLAGS) src/lib.rs
	./build/dir

clean:
	git clean -df
