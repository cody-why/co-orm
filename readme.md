# co-orm

[![Crates.io](https://img.shields.io/crates/v/co-orm.svg)](https://crates.io/crates/co-orm)
[![Docs](https://docs.rs/co-orm/badge.svg)](https://docs.rs/co-orm)
[![Download](https://img.shields.io/crates/d/co-orm.svg?style=flat-square)](https://crates.io/crates/co-orm)

A high-performance, async CRUD library for sqlx that provides elegant database operations with compile-time safety.

## Features

- **Zero-cost abstractions** - Minimal runtime overhead
- **Type-safe queries** - Compile-time SQL generation
- **Async-first** - Built on tokio and sqlx
- **Flexible filtering** - Powerful WHERE clause builder
- **Multiple databases** - MySQL, PostgreSQL, SQLite support
- **Macro-driven** - Simple derive macros for common operations

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
co-orm = { version = "0.3", features = ["mysql"] }
sqlx = { version = "0.8", features = ["mysql", "runtime-tokio-native-tls"] }
```

Available features: `mysql`, `postgres`, `sqlite`

## Quick Start

### Basic Model Definition

```rust
use co_orm::{Crud, sqlx::FromRow};
use chrono::NaiveDateTime;

#[derive(Debug, Crud, FromRow)]
#[co_orm(rename = "users")]
struct User {
    #[co_orm(skip_insert)]
    pub id: i64,
    #[co_orm(rename = "name")]
    pub name: String,
    #[co_orm(update)]
    pub password: String,
    #[co_orm(skip)]
    #[sqlx(skip)]
    pub addr: Option<String>,
    pub age: Option<u32>,
    #[co_orm(skip_insert)]
    pub update_at: Option<NaiveDateTime>,
}
```

### Basic CRUD Operations

```rust
use co_orm::{Crud, Where};

// Get by primary key
let user = User::get(&pool, 1).await?;

// Query all
let users = User::query(&pool).await?;

// Insert
let new_user = User { /* ... */ };
new_user.insert(&pool).await?;

// Update
user.update(&pool).await?;

// Delete
user.delete(&pool).await?;
```

### Advanced Querying

```rust
// Simple conditions
let users = User::query_where(&pool, Where::new()
    .eq("status", "active")
    .and()
    .ge("age", 18)
).await?;

// Complex grouped conditions
let users = User::query_where(&pool, Where::new()
    .eq("status", "active")
    .and_group(|w| {
        w.gt("age", 18)
         .and()
         .lt("age", 65)
    })
    .or_group(|w| {
        w.eq("name", "admin")
         .and()
         .ge("age", 21)
    })
).await?;

// Pagination
let (count, users) = User::query_page_where(
    &pool,
    Where::new().eq("status", "active"),
    1,  // page
    10  // page_size
).await?;
```

## Available Methods

The `#[derive(Crud)]` macro generates these methods:

- **Read**: `get()`, `get_by()`, `get_where()`, `query()`, `query_by()`, `query_where()`
- **Write**: `insert()`, `insert_all()`, `update()`, `update_by()`, `update_where()`
- **Delete**: `delete()`, `delete_by()`, `delete_where()`
- **Pagination**: `query_page_by()`, `query_page_where()`

## Field Attributes

| Attribute | Description |
|-----------|-------------|
| `#[co_orm(id)]` | Mark as primary key (default: first field), for update and delete. |
| `#[co_orm(skip_insert)]` | Skip field during insert operations |
| `#[co_orm(rename = "name")]` | Rename table or field in database |
| `#[co_orm(skip)]` | Ignore field completely |
| `#[co_orm(update)]` | Generate update methods for this field |

## Query Operators

| Method | SQL | Description |
|--------|-----|-------------|
| `eq(col, value)` | `=` | Equal |
| `ne(col, value)` | `<>` | Not equal |
| `gt(col, value)` | `>` | Greater than |
| `lt(col, value)` | `<` | Less than |
| `ge(col, value)` | `>=` | Greater or equal |
| `le(col, value)` | `<=` | Less or equal |
| `like(col, value)` | `LIKE` | Pattern matching |
| `between(col, start, end)` | `BETWEEN` | Range query |
| `r#in(col, values)` | `IN` | Set membership |
| `is_null(col)` | `IS NULL` | Null check |
| `is_not_null(col)` | `IS NOT NULL` | Not null check |
| `raw(fragment)` | Custom | Custom SQL fragment |

## Logical Operators

| Method | Description |
|--------|-------------|
| `and()` | AND logic |
| `or()` | OR logic |
| `and_group(f)` | AND grouping |
| `or_group(f)` | OR grouping |

## Utility Macros

```rust
// Query arguments
let args = args!(&name, age);

// Pagination arguments  
let args = page_args!(&name, age);

// Raw queries
query!("INSERT INTO users (name, password) VALUES (?, ?)", name, password)
    .execute(&pool).await?;

// Typed queries
query_as!(User, "SELECT * FROM users WHERE name = ?", name)
    .fetch_one(&pool).await?;
```

## Examples

See the `examples/` directory for complete working examples:

- `crud.rs` - Basic CRUD operations
- `where_examples.rs` - Advanced querying patterns
- `pool.rs` - Connection pool management

## License

Licensed under the MIT License.