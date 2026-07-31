Para usar diesel usa la flag --migration-dir migrations/sqlite

Para hacer toda la base de datos se hace:
diesel migration run --migration-dir migrations/sqlite

Y Para rehacer toda la base de datos se haría:
diesel migration revert --all --migration-dir migrations/sqlite && diesel migration run --migration-dir migrations/sqlite

Para ejecutar cli hay que hacer:
cargo run -p cli -- --config-path=./../config.toml record