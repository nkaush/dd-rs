debug:
	cargo build
	mv target/debug/dd-rs .

release:
	cargo build --release
	mv target/release/dd-rs .