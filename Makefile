run-db:
	docker-compose -f docker/docker-compose.yaml up db pgadmin -d

run-flashcard:
	RUST_BACKTRACE=full RUST_LOG=trace,hyper=info${RUST_LOG} cargo run --bin flashcard

rollback-migration:
	sea-orm-cli migrate down -n 11

run-migration:
	sea-orm-cli migrate up

generate-entities:
	sea-orm-cli generate entity --output-dir ./src/entities --with-serde both

redo-migration: rollback-migration run-migration generate-entities