set dotenv-load := true

run: tailwind
	cargo run

dev:
	cargo watch -s "just build-dev"

build-dev: tailwind
	cargo run

tailwind:
	npx @tailwindcss/cli --i styles/style.css --o public/style.css

psql:
	psql "dbname=$PGDATABASE host=$PGHOST user=$PGUSER password=$PGPASSWORD port=$PGPORT"
