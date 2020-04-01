use actix::{Actor, SyncContext};
use diesel::{PgConnection}

pub struct DbExecutor {
    conn: PgConnection,
}

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

impl DbExecutor {
    pub fn new(db_url: &str) -> DbExecutor {
        DbExecutor {
            conn: PgConnection::establish(db_url)
                    .expect(&format!("Error connecting to {}", db_url))
        }
    }
}
