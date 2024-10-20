use sqlx::{Pool, Sqlite};

#[derive(Clone, Debug)]
pub struct State {
    pool: Pool<Sqlite>,
}

impl State {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
}
