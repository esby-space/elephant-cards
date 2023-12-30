# elephant cards
an [Anki](https://apps.ankiweb.net/) clone, except it runs on the web. and it has about 0.2% of the features. and it's not even finished. so it's just worse !

## starting

required:
- git
- cargo
- sqlite3

```shell
git clone https://github.com/esby-space/elephant-cards.git
cd elephant-cards

cargo install sqlx-cli
export DATABASE_URL="sqlite:db/cards.db"
mkdir db
cargo sqlx database create
cargo sqlx migrate run

cargo run
```

## the stack
(idk what i'm doin', don't judge)

- frontend
    - HTMX
    - TailwindCSS
- backend
    - Axum
    - SQLx
    - Maud

the MATHS stack? oh pls no, anything but this

