use diesel_async::pooled_connection::bb8;
use diesel_async::AsyncPgConnection;

use crate::config::Config;

pub struct Global {
    pub config: Config,
    pub database: bb8::Pool<AsyncPgConnection>,
}
