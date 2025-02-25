run-db:
	docker-compose -f docker/docker-compose.yaml up db -d

run-flashcard:
	cargo run flashcard