# Need `cargo install cargo-watch`
run_server:
	cargo watch -q -c -w src/ -x run 

run_client:
	cargo watch -q -c -w tests/ -x "test -- --nocapture"

test_login:
	cargo watch -q -c -w tests/ -x "test --test routes_login -- --nocapture"

test_crud_ticket:
	cargo watch -q -c -w tests/ -x "test --test crud_tickets -- --nocapture"

test_auth_middleware_succeed:
	cargo watch -q -c -w tests/ -x "test --test auth_middleware_succeed -- --nocapture"

test_auth_middleware_failed:
	cargo watch -q -c -w tests/ -x "test --test auth_middleware_failed -- --nocapture"