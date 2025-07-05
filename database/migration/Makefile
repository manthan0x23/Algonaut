generate-entity:
	sea-orm-cli generate entity -o ../src/entity/

migrate-db:
	RUST_LOG=trace cargo run

create-migration:
	@if [ -z "$(name)" ]; then \
		echo "Usage: make create-migration name=your_migration_name"; \
		exit 1; \
	else \
		cd .. && sea-orm-cli migrate generate $(name); \
	fi