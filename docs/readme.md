# Usage
## SQLite
Para usar diesel usa la flag --migration-dir migrations/sqlite

Para hacer toda la base de datos se hace:
```bash
diesel migration run --migration-dir migrations/sqlite
```

Y Para rehacer toda la base de datos se haría:
```bash
diesel migration revert --all --migration-dir migrations/sqlite && diesel migration run --migration-dir migrations/sqlite
```

Después de tener la base de datos ejecuta [Common execution](#common-execution)

## Odoo
Debe tener una instancia corriendo y ejecutar y ejecuta [Common execution](#common-execution)

## Common execution
```bash
cargo run -p cli -- --config-path=./../config.toml record
```

# Configurations
## Storage
### SQLite
```toml
[storage]

[storage.sqlite]
database_url = "./../testing.db"
```