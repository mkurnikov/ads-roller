## Compilation

Requires **nightly** compiler (for Rocket). 

## How to run

```
# start db instance
docker-compose up -d

# install diesel cli to perform migrations
cargo install diesel_cli

# migrate and populate db
diesel migration run
cargo run --bin prepare_db

# start webserver
cargo run --bin web
```

## How to use

Open `http://localhost:8000/` in browser, 
passing optional `?category1=NAME&category2=NAME&category3=NAME`

If no parameters passed, ad will be sampled from all records with paid shows != 0.
 
Only three categories are supported, due to limitations in https://github.com/SergioBenitez/Rocket/issues/205 