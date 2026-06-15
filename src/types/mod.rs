use sqlx::{MySql, Pool};

use crate::services::logger::Logger;
#[derive(serde::Serialize)]
pub struct ApiResponse<T>
where
    T: serde::Serialize + Send + Sync + 'static,
{
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T>
where
    T: serde::Serialize + Send + Sync + 'static,
{
    pub fn new(message: &str, data: Option<T>) -> Self {
        Self {
            message: message.to_string(),
            data,
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<MySql>,
    pub logger: Logger,
}
