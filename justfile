run: tailwind
	cargo run

dev:
	cargo watch -s "just build-dev"

build-dev: tailwind
	cargo run

tailwind:
	npx @tailwindcss/cli --i styles/style.css --o public/style.css
