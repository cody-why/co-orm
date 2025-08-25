#[cfg(feature = "mysql")]
type DbArgs = sqlx::mysql::MySqlArguments;
#[cfg(all(
    not(feature = "mysql"),
    not(feature = "postgres"),
    not(feature = "sqlite"),
    // not(feature = "mssql"),
))]
type DbArgs = sqlx::mysql::MySqlArguments; // default to MySQL like macros crate
#[cfg(feature = "postgres")]
type DbArgs = sqlx::postgres::PgArguments;
#[cfg(feature = "sqlite")]
type DbArgs = sqlx::sqlite::SqliteArguments;
// #[cfg(feature = "mssql")]
// type DbArgs = sqlx::mssql::MssqlArguments;

pub trait BindArg {
    fn bind_to(self, args: &mut DbArgs);
}

#[cfg(feature = "mysql")]
impl<T> BindArg for T
where
    T: 'static + sqlx::Encode<'static, sqlx::MySql> + sqlx::Type<sqlx::MySql>,
{
    fn bind_to(self, args: &mut DbArgs) {
        use sqlx::Arguments as _;
        let _ = args.add(self);
    }
}

#[cfg(feature = "postgres")]
impl<T> BindArg for T
where
    T: 'static + sqlx::Encode<'static, sqlx::Postgres> + sqlx::Type<sqlx::Postgres>,
{
    fn bind_to(self, args: &mut DbArgs) {
        use sqlx::Arguments as _;
        let _ = args.add(self);
    }
}

#[cfg(feature = "sqlite")]
impl<T> BindArg for T
where
    T: 'static + sqlx::Encode<'static, sqlx::Sqlite> + sqlx::Type<sqlx::Sqlite>,
{
    fn bind_to(self, args: &mut DbArgs) {
        use sqlx::Arguments as _;
        let _ = args.add(self);
    }
}

// #[cfg(feature = "mssql")]
// impl<T> BindArg for T
// where
//     T: 'static + sqlx::Encode<'static, sqlx::Mssql> + sqlx::Type<sqlx::Mssql>,
// {
//     fn bind_to(self, args: &mut DbArgs) {
//         use sqlx::Arguments as _;
//         let _ = args.add(self);
//     }
// }

/// A builder for WHERE clauses
/// # Example:
/// ```ignore
/// let w = Where::new().eq("name", "admin").and().ge("age", 18);
/// ```
/// will generate:
/// ```ignore
/// WHERE name = ? AND age >= ?
/// ```
#[derive(Debug, Default, Clone)]
pub struct Where {
    sql: String,
    args: DbArgs,
    next_index: usize,
    pending_logic: Option<&'static str>,
    has_any_predicate: bool,
    skip_next_logic: bool,
}

impl Where {
    /// Create a new Where builder
    /// # Example:
    /// ```ignore
    /// let w = Where::new();
    /// ```
    pub fn new() -> Self {
        Self {
            sql: String::from("WHERE "),
            args: Default::default(),
            next_index: 1,
            pending_logic: None,
            has_any_predicate: false,
            skip_next_logic: false,
        }
    }

    /// Add AND logic to the WHERE clause
    /// # Example:
    /// ```ignore
    /// Where::new().eq("name", "admin").and().eq("age", 18);
    /// ```
    /// will generate:
    /// ```ignore
    /// WHERE name = ? AND age = ?
    /// ```
    pub fn and(mut self) -> Self {
        self.pending_logic = Some("AND");
        self
    }

    /// Add OR logic to the WHERE clause
    /// # Example:
    /// ```ignore
    /// Where::new().eq("name", "admin").or().eq("age", 18);
    /// ```
    /// will generate:
    /// ```ignore
    /// WHERE name = ? OR age = ?
    /// ```
    pub fn or(mut self) -> Self {
        self.pending_logic = Some("OR");
        self
    }

    /// Push logic if needed
    fn push_logic_if_needed(&mut self) {
        if self.skip_next_logic {
            self.skip_next_logic = false;
            return;
        }
        if self.has_any_predicate {
            let logic = self.pending_logic.take().unwrap_or("AND");
            self.sql.push_str(logic);
            self.sql.push(' ');
        }
    }

    /// Generate a placeholder for the next value
    fn placeholder(&mut self) -> String {
        #[cfg(feature = "postgres")]
        {
            let p = format!("${}", self.next_index);
            self.next_index += 1;
            return p;
        }
        #[cfg(feature = "sqlite")]
        {
            let p = format!("?{}", self.next_index);
            self.next_index += 1;
            return p;
        }
        #[cfg(feature = "mysql")]
        {
            self.next_index += 1;
            return "?".to_string();
        }
        // #[cfg(feature = "mssql")]
        // {
        //     self.next_index += 1;
        //     return "?".to_string();
        // }
        #[allow(unreachable_code)]
        {
            self.next_index += 1;
            "?".to_string()
        }
    }

    /// Add a raw SQL fragment to the WHERE clause
    /// # Example:
    /// ```ignore
    /// Where::new().raw("name = admin AND age >= 18");
    /// ```
    pub fn raw(mut self, fragment: &str) -> Self {
        self.push_logic_if_needed();
        self.sql.push_str(fragment);
        self.sql.push(' ');
        self.has_any_predicate = true;
        self
    }

    /// Add an equality condition(=) to the WHERE clause
    /// # Example:
    /// ```ignore
    /// Where::new().eq("name", "admin");
    /// ```
    /// will generate:
    /// ```ignore
    /// WHERE name = ?
    /// ```
    pub fn eq(mut self, col: &str, value: impl BindArg) -> Self {
        self.push_logic_if_needed();
        let ph = self.placeholder();
        self.sql.push_str(col);
        self.sql.push_str(" = ");
        self.sql.push_str(&ph);
        self.sql.push(' ');
        self.has_any_predicate = true;
        value.bind_to(&mut self.args);
        self
    }

    /// Add a non-equality condition(<>) to the WHERE clause
    /// # Example:
    /// ```ignore
    /// Where::new().ne("name", "admin");
    /// ```
    /// will generate:
    /// ```ignore
    /// WHERE name <> ?
    /// ```
    pub fn ne(mut self, col: &str, value: impl BindArg) -> Self {
        self.push_logic_if_needed();
        let ph = self.placeholder();
        self.sql.push_str(col);
        self.sql.push_str(" <> ");
        self.sql.push_str(&ph);
        self.sql.push(' ');
        self.has_any_predicate = true;
        value.bind_to(&mut self.args);
        self
    }

    /// Add a less than condition(<) to the WHERE clause
    /// # Example:
    /// ```ignore
    /// Where::new().lt("age", 18);
    /// ```
    /// will generate:
    /// ```ignore
    /// WHERE age < ?
    /// ```
    pub fn lt(mut self, col: &str, value: impl BindArg) -> Self {
        self.push_logic_if_needed();
        let ph = self.placeholder();
        self.sql.push_str(col);
        self.sql.push_str(" < ");
        self.sql.push_str(&ph);
        self.sql.push(' ');
        self.has_any_predicate = true;
        value.bind_to(&mut self.args);
        self
    }

    /// Add a less than or equal to condition(<=) to the WHERE clause
    /// # Example:
    /// ```ignore
    /// Where::new().le("age", 18);
    /// ```
    /// will generate:
    /// ```ignore
    /// WHERE age <= ?
    /// ```
    pub fn le(mut self, col: &str, value: impl BindArg) -> Self {
        self.push_logic_if_needed();
        let ph = self.placeholder();
        self.sql.push_str(col);
        self.sql.push_str(" <= ");
        self.sql.push_str(&ph);
        self.sql.push(' ');
        self.has_any_predicate = true;
        value.bind_to(&mut self.args);
        self
    }

    /// Add a greater than condition(>) to the WHERE clause
    /// # Example:
    /// ```ignore
    /// Where::new().gt("age", 18);
    /// ```
    /// will generate:
    /// ```ignore
    /// WHERE age > ?
    /// ```
    pub fn gt(mut self, col: &str, value: impl BindArg) -> Self {
        self.push_logic_if_needed();
        let ph = self.placeholder();
        self.sql.push_str(col);
        self.sql.push_str(" > ");
        self.sql.push_str(&ph);
        self.sql.push(' ');
        self.has_any_predicate = true;
        value.bind_to(&mut self.args);
        self
    }

    /// Add a greater than or equal to condition(>=) to the WHERE clause
    /// # Example:
    /// ```ignore
    /// Where::new().ge("age", 18);
    /// ```
    /// will generate:
    /// ```ignore
    /// WHERE age >= ?
    /// ```
    pub fn ge(mut self, col: &str, value: impl BindArg) -> Self {
        self.push_logic_if_needed();
        let ph = self.placeholder();
        self.sql.push_str(col);
        self.sql.push_str(" >= ");
        self.sql.push_str(&ph);
        self.sql.push(' ');
        self.has_any_predicate = true;
        value.bind_to(&mut self.args);
        self
    }

    /// Add a LIKE condition to the WHERE clause
    /// # Example:
    /// ```ignore
    /// Where::new().like("name", "%admin%");
    /// ```
    /// will generate:
    /// ```ignore
    /// WHERE name LIKE ?
    /// ```
    pub fn like(mut self, col: &str, value: impl BindArg) -> Self {
        self.push_logic_if_needed();
        let ph = self.placeholder();
        self.sql.push_str(col);
        self.sql.push_str(" LIKE ");
        self.sql.push_str(&ph);
        self.sql.push(' ');
        self.has_any_predicate = true;
        value.bind_to(&mut self.args);
        self
    }

    /// Add a IS NULL condition to the WHERE clause
    /// # Example:
    /// ```ignore
    /// Where::new().is_null("status");
    /// ```
    /// will generate:
    /// ```ignore
    /// WHERE status IS NULL
    /// ```
    pub fn is_null(mut self, col: &str) -> Self {
        self.push_logic_if_needed();
        self.sql.push_str(col);
        self.sql.push_str(" IS NULL ");
        self.has_any_predicate = true;
        self
    }

    /// Add a IS NOT NULL condition to the WHERE clause
    /// # Example:
    /// ```ignore
    /// Where::new().is_not_null("status");
    /// ```
    /// will generate:
    /// ```ignore
    /// WHERE status IS NOT NULL
    /// ```
    pub fn is_not_null(mut self, col: &str) -> Self {
        self.push_logic_if_needed();
        self.sql.push_str(col);
        self.sql.push_str(" IS NOT NULL ");
        self.has_any_predicate = true;
        self
    }

    /// Add a BETWEEN condition to the WHERE clause
    /// # Example:
    /// ```ignore
    /// Where::new().between("age", 18, 21);
    /// ```
    /// will generate:
    /// ```ignore
    /// WHERE age BETWEEN ? AND ?
    /// ```
    pub fn between(mut self, col: &str, start: impl BindArg, end: impl BindArg) -> Self {
        self.push_logic_if_needed();
        let ph1 = self.placeholder();
        let ph2 = self.placeholder();
        self.sql.push_str(col);
        self.sql.push_str(" BETWEEN ");
        self.sql.push_str(&ph1);
        self.sql.push_str(" AND ");
        self.sql.push_str(&ph2);
        self.sql.push(' ');
        self.has_any_predicate = true;
        start.bind_to(&mut self.args);
        end.bind_to(&mut self.args);
        self
    }

    /// Add a IN condition to the WHERE clause
    /// # Example:
    /// ```ignore
    /// Where::new().r#in("status", vec!["active", "verified", "premium"]);
    /// ```
    /// will generate:
    /// ```ignore
    /// WHERE status IN (?, ?, ?)
    /// ```
    pub fn r#in<V>(mut self, col: &str, values: V) -> Self
    where
        V: IntoIterator,
        V::Item: BindArg,
    {
        self.push_logic_if_needed();
        self.sql.push_str(col);
        self.sql.push_str(" IN (");
        let mut first = true;
        for v in values {
            if !first {
                self.sql.push_str(", ");
            }
            first = false;
            let ph = self.placeholder();
            self.sql.push_str(&ph);
            v.bind_to(&mut self.args);
        }
        self.sql.push(')');
        self.sql.push(' ');
        self.has_any_predicate = true;
        self
    }

    /// Add a AND group to the WHERE clause
    /// # Example:
    /// ```ignore
    /// Where::new().eq("name", "admin").and_group(|w| {
    ///     w.eq("password", "123456").and().ge("age", 21);
    /// });
    /// ```
    /// will generate:
    /// ```ignore
    /// WHERE name = ? AND (password = ? AND age >= ?)
    /// ```
    pub fn and_group<F>(mut self, f: F) -> Self
    where
        F: FnOnce(Where) -> Where,
    {
        self.pending_logic = Some("AND");
        self.push_logic_if_needed();
        self.sql.push('(');
        let prev_skip = self.skip_next_logic;
        self.skip_next_logic = true;
        let mut new_where = f(self);
        new_where.skip_next_logic = prev_skip;
        if new_where.sql.ends_with(' ') {
            new_where.sql.pop();
        }
        new_where.sql.push(')');
        new_where.sql.push(' ');
        new_where.has_any_predicate = true;
        new_where
    }

    /// Add a OR group to the WHERE clause
    /// # Example:
    /// ```ignore
    /// Where::new().eq("name", "root").or_group(|w| {
    ///     w.eq("name", "admin").and().ge("age", 21);
    /// });
    /// ```
    /// will generate:
    /// ```ignore
    /// WHERE name = ? AND (age >= ? OR name = ?)
    /// ```
    pub fn or_group<F>(mut self, f: F) -> Self
    where
        F: FnOnce(Where) -> Where,
    {
        self.pending_logic = Some("OR");
        self.push_logic_if_needed();
        self.sql.push('(');
        let prev_skip = self.skip_next_logic;
        self.skip_next_logic = true;
        let mut new_where = f(self);
        new_where.skip_next_logic = prev_skip;
        if new_where.sql.ends_with(' ') {
            new_where.sql.pop();
        }
        new_where.sql.push(')');
        new_where.sql.push(' ');
        new_where.has_any_predicate = true;
        new_where
    }

    /// Build the WHERE clause
    /// # Example:
    /// ```ignore
    /// let (sql, args) = Where::new().build();
    /// ```
    pub fn build(mut self) -> (String, DbArgs) {
        if !self.has_any_predicate {
            return (String::new(), self.args);
        }
        if self.sql.ends_with(' ') {
            self.sql.pop();
        }
        (self.sql, self.args)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_where() {
        let w = Where::new().eq("status", "active").and().ge("age", 18).or().le("age", 21);
        let (sql, args) = w.build();
        assert_eq!(sql, "WHERE status = ? AND age >= ? OR age <= ?");
        println!("args: {:?}", args);

        let w = Where::new().gt("age", 18).and().lt("age", 21);
        let (sql, _args) = w.build();
        assert_eq!(sql, "WHERE age > ? AND age < ?");

        let w = Where::new().between("age", 18, 21);
        let (sql, _args) = w.build();
        assert_eq!(sql, "WHERE age BETWEEN ? AND ?");

        let w = Where::new().r#in("status", vec!["active", "verified", "premium"]);
        let (sql, _args) = w.build();
        assert_eq!(sql, "WHERE status IN (?, ?, ?)");

        let w = Where::new().is_null("status");
        let (sql, _args) = w.build();
        assert_eq!(sql, "WHERE status IS NULL");

        let w = Where::new().is_not_null("status");
        let (sql, _args) = w.build();
        assert_eq!(sql, "WHERE status IS NOT NULL");

        let w = Where::new().like("name", "%admin%");
        let (sql, _args) = w.build();
        assert_eq!(sql, "WHERE name LIKE ?");
    }

    #[test]
    fn test_where_and_group() {
        let w = Where::new()
            .eq("status", "active")
            .and_group(|w| w.ge("age", 18).or().eq("name", "root"));
        let (sql, _args) = w.build();
        assert_eq!(sql, "WHERE status = ? AND (age >= ? OR name = ?)");
    }

    #[test]
    fn test_where_or_group() {
        let w = Where::new()
            .eq("name", "root")
            .or_group(|w| w.eq("name", "admin").and().ge("age", 21));
        let (sql, _args) = w.build();
        assert_eq!(sql, "WHERE name = ? OR (name = ? AND age >= ?)");
    }
}
