# elephant cards
a worse [Anki](https://apps.ankiweb.net/) clone.

## the stack
(idk what i'm doin', don't judge)

- frontend
    - HTMX
    - TailwindCSS
- backend
    - Axum
    - Askama
    - SQLX

SATHA?

## starting

```
cargo install sqlx-cli
export DATABASE_URL="sqlite:db/cards.db"
sqlx database create
sqlx migrate run
cargo run
```

