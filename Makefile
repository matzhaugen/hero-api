.PHONY: pg setup prod benchmark
pg:
	docker run \
    --name pg \
    -e POSTGRES_PASSWORD=one \
    -p 5432:5432 \
   	-v `pwd`/init-pg-db.sql:/docker-entrypoint-initdb.d/init-user-db.sql \
    postgres:11
setup:
	DATABASE_URL=postgresql://hero:one@127.0.0.1:5432/heroes diesel setup &&\
	diesel migration run
prod:
	cargo build --release && ROCKET_ENV=prod ./target/release/hero-api
benchmark:
	drill -b benchmark.yml -s