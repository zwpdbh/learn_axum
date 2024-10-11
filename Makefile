# Need `cargo install cargo-watch`
run_server:
	cargo watch -q -c -w src/ -x run 

run_client:
	cargo watch -q -c -w tests/ -x "test -- --nocapture"