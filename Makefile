.PHONY: build install clean

build:
	rm -rf target
	cargo build --release
	upx --best --lzma target/release/nyamote

install: build
	cp target/release/nyamote ~/.cargo/bin/

clean:
	cargo clean
