release:
	cargo build --release
	mv target/release/dd-rs .

debug:
	cargo build
	mv target/debug/dd-rs .

clean:
	cargo clean