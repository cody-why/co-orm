# Changelog

## 0.3.12 - 2024-08-3
* [[#1]]: fix: features sqllite compilation error, remove default features.
* postgres: sqlx `PgArguments` not support `Clone` trait, so need use `page_args!()` for `query_page_by`.