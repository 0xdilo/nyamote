.PHONY: build install clean

build:
	cargo build --release
	upx --best --lzma target/release/namote

install: build
	cp target/release/namote ~/.cargo/bin/

clean:
	cargo clean
