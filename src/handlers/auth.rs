pub struct AuthHandler;

impl AuthHandler {
    pub async fn log_in() -> String {
        "log in".to_string()
    }
    pub async fn sign_in() -> String {
        "Sign in".to_string()
    }
    pub async fn log_out() -> String {
        "Log out".to_string()
    }
}
