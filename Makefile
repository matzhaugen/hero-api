.PHONY: pg setup prod benchmark
pg:
	docker run \
    --name pg \
    -e POSTGRES_PASSWORD=one \
    --network host \
    -e DB_HOST=docker.for.mac.host.internal \
   	-v `pwd`/init-pg-db.sql:/docker-entrypoint-initdb.d/init-user-db.sql \
    postgres:11
setup:
	DATABASE_URL=postgresql://hero:one@127.0.0.1:5432/heroes diesel setup &&\
	diesel migration run
prod:
	cargo build --release && ROCKET_ENV=prod ./target/release/hero-api
benchmark:
	drill -b benchmark.yml -s