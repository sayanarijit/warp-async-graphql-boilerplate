.PHONY: watch
watch:
	cargo build
	env RUST_LOG=warp=TRACE cargo watch -x run

db:
	# http://localhost:8081/?pgsql=db&username=postgres&db=postgres
	docker-compose up db adminer
