build:
	cargo build

run-server:
	cd server && cargo run 

run-worker:
	cd worker && cargo run

run-client:
	cd client && pnpm run dev