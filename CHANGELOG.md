# Changelog

## 0.3.12
* 1. fix: features sqllite compilation error, remove default features.
* 2. postgres: sqlx `PgArguments` not support `Clone` trait, so need use `page_args!()` for `query_page_by`.

## 0.3.13 
* 1. feat: add `Where` struct for build sql where clause.
* 2. feat: generate `get_where`, `query_where`, `update_where`, `delete_where`, `query_page_where` method.
* 3. feat: remove `#[co_orm(by)]` attribute.
* 4. feat: remove `#[co_orm(seq)]` attribute, use `#[co_orm(skip_insert)]` instead.

## 0.3.15
* 1. fix: `update_where` not support `Arguments` for update, remove `update_where` method.