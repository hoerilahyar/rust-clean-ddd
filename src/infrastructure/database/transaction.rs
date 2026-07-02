use sqlx::{MySql, Transaction};

pub type DbTransaction<'a> = Transaction<'a, MySql>;
