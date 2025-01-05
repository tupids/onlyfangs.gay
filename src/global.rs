use diesel_async::{pooled_connection::bb8, AsyncPgConnection};

use crate::config::Config;

pub struct Global {
    pub config: Config,
    pub database: bb8::Pool<AsyncPgConnection>,
}
