# hero-api

1. Set up database run `make pg && make setup` 
2. Change the relvant line in `schema.rs` to `id -> Nullable<Int4>,`  
3. Run the app do `make prod` see [link](https://stackoverflow.com/questions/31249112/allow-docker-container-to-connect-to-a-local-host-postgres-database) for more info on running db on host network

###  Other good links 
https://k6.io/blog/comparing-best-open-source-load-testing-tools
