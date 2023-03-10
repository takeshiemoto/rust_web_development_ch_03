build:
	docker-compose build

db:
	docker-compose up

test:
	cargo test

# standalone test
test-s:
	cargo test --no-default-features