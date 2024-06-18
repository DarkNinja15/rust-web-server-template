# Connecting Postgresql

## Dependencies

```toml
[dependencies]
axum = "0.5"
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
dotenv = "0.15"

[dependencies.diesel]
version = "2.0.0"
features = ["postgres", "r2d2", "chrono"]

[dependencies]
r2d2 = "0.8.9"
r2d2-diesel = "1.0.0"
```

## Install distel_cli

```bash
cargo install diesel_cli --no-default-features --features postgres
```

## Set up diesel

```bash
diesel setup
```
This will create a `diesel.toml` file and a `migrations` directory in the project.

## Configure env

Create .env at root

```env
DATABASE_URL=postgres://username:password@localhost/mydatabase
```

## Create migration

create your models

```bash
diesel migration generate <migration_name>
```

## Edit migrations

Edit the migration file in `migrations/<migration_name>/up.sql`

```sql
-- migrations/YYYYMMDDHHMMSS_create_users/up.sql
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    email VARCHAR NOT NULL UNIQUE
);

-- migrations/YYYYMMDDHHMMSS_create_users/down.sql
DROP TABLE users;
```
Example for users model

## Run migration

```bash
diesel migration run
```


