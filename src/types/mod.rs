#[derive(serde::Serialize)]
pub struct ApiResponse<T>
where
    T: serde::Serialize + Send + Sync + 'static,
{
    pub message: String,
    pub data: Option<T>,
}



use sqlx::{MySql, Pool};

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<MySql>,
}
