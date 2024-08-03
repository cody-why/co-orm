/*
 * @Date: 2024-08-03 23:52:16
 * @LastEditTime: 2024-08-03 23:56:06
 */

/// sqlx::query_as
/// ``` no_run,ignore
/// query_as!(User, "select * from users where name = ?", name).fetch_one(&pool).await
/// ```
#[macro_export]
macro_rules! query_as (
    ($out_struct:path, $query:expr) => ( {
        sqlx::query_as::<_, $out_struct>($query)
    });
    ($out_struct:path, $query:expr, $($args:expr),*) => ( {
        sqlx::query_as::<_, $out_struct>($query)
        $(.bind($args))*
    })
);

/// sqlx::query
/// ``` no_run,ignore
/// query!("insert into users (name, password) values (?,?)", name, password).execute(&pool).await
/// ```
#[macro_export]
macro_rules! query (
    ($query:expr) => ( {
        sqlx::query($query)
    });
    ($query:expr, $($args:expr),*) => ( {
        sqlx::query($query)
        $(.bind($args))*
    })
);

#[cfg(feature = "mysql")]
#[macro_export]
macro_rules! args {
    // ($sql:expr) => {
    //     args!($sql,);
    // };

    ($($args:expr),*) => {{
        use sqlx::Arguments;
        let mut sqlargs = sqlx::mysql::MySqlArguments::default();
        $(sqlargs.add($args);)*
        sqlargs


    }};
}

#[cfg(feature = "postgres")]
#[macro_export]
macro_rules! args {
    ($($args:expr),*) => {{
        use sqlx::Arguments;
        let mut sqlargs = sqlx::postgres::PgArguments::default();
        $(sqlargs.add($args);)*
        sqlargs
    }};

}

#[cfg(feature = "sqlite")]
#[macro_export]
macro_rules! args {
    ($($args:expr),*) => {{
        use sqlx::Arguments;
        let mut sqlargs = sqlx::sqlite::SqliteArguments::default();
        $(sqlargs.add($args);)*
        sqlargs
    }};
}

#[cfg(feature = "mssql")]
#[macro_export]
macro_rules! args {
    ($($args:expr),*) => {{
        use sqlx::Arguments;
        let mut sqlargs = sqlx::mssql::MssqlArguments::default();
        $(sqlargs.add($args);)*
        sqlargs
    }};

}

#[cfg(feature = "mysql")]
#[macro_export]
macro_rules! page_args {
    ($($args:expr),*) => {{
        use sqlx::Arguments;
        let mut sqlargs = sqlx::mysql::MySqlArguments::default();
        $(sqlargs.add($args);)*
        let mut sqlargs2 = sqlx::mysql::MySqlArguments::default();
        $(sqlargs2.add($args);)*
        (sqlargs, sqlargs2)


    }};
}

#[cfg(feature = "postgres")]
#[macro_export]
macro_rules! page_args {
    ($($args:expr),*) => {{
        use sqlx::Arguments;
        let mut sqlargs = sqlx::postgres::PgArguments::default();
        $(sqlargs.add($args);)*
        let mut sqlargs2 = sqlx::postgres::PgArguments::default();
        $(sqlargs2.add($args);)*
        (sqlargs, sqlargs2)
    }};

}

#[cfg(feature = "sqlite")]
#[macro_export]
macro_rules! page_args {
    ($($args:expr),*) => {{
        use sqlx::Arguments;
        let mut sqlargs = sqlx::sqlite::SqliteArguments::default();
        $(sqlargs.add($args);)*
        let mut sqlargs2 = sqlx::sqlite::SqliteArguments::default();
        $(sqlargs2.add($args);)*
        (sqlargs, sqlargs2)
    }};
}

#[cfg(feature = "mssql")]
#[macro_export]
macro_rules! page_args {
    ($($args:expr),*) => {{
        use sqlx::Arguments;
        let mut sqlargs = sqlx::mssql::MssqlArguments::default();
        $(sqlargs.add($args);)*
        let mut sqlargs2 = sqlx::mssql::MssqlArguments::default();
        $(sqlargs2.add($args);)*
        (sqlargs, sqlargs2)
    }};

}
