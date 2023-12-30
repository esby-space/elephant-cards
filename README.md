# elephant cards
a worse [Anki](https://apps.ankiweb.net/) clone.

## the stack
(idk what i'm doin', don't judge)

- frontend
    - HTMX
    - TailwindCSS
- backend
    - Axum
    - SQLX

SATHA?

## starting

```
export DATABASE_URL="sqlite:db/cards.db"
mkdir db
cargo sqlx database create
cargo sqlx migrate run
cargo run
```

