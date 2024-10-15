# Need `cargo install cargo-watch`
run_server:
	cargo watch -q -c -w src/ -w .cargo/ -x run 

run_client:
	cargo watch -q -c -w examples/ -x "run --example quick_dev"

# test_login:
# 	cargo watch -q -c -w tests/ -x "test --test routes_login -- --nocapture"

# test_crud_ticket:
# 	cargo watch -q -c -w tests/ -x "test --test crud_tickets -- --nocapture"

# test_auth_middleware_succeed:
# 	cargo watch -q -c -w tests/ -x "test --test auth_middleware_succeed -- --nocapture"

# test_auth_middleware_failed:
# 	cargo watch -q -c -w tests/ -x "test --test auth_middleware_failed -- --nocapture"


# Starting the DB
# Start postgresql server docker image:
run_db_dev: 
	docker run --rm --name pg -p 5432:5432 -e POSTGRES_PASSWORD=welcome postgres:16
run_docker_file_for_dev:
	

# (optional) To have a psql terminal on pg. 
# In another terminal (tab) run psql:
run_psql_terminal:
	docker exec -it -u postgres pg psql
