## Database tooling

assuming you have docker already

install psql and sqlx-cli:

```bash
sudo apt-get install -y postgresql-client
cargo install sqlx-cli
```

See ```.env``` for connection string

To start db:

```bash
docker compose up
```

To connect to db with psql:
```bash
psql -h localhost -U admin -d blackbird
```

To run migrations

```bash
cargo sqlx migrate run
```

To prepare sqlx macros
```bash
cargo sqlx prepare -- --all-targets --all-features
```