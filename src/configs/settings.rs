pub struct Variables;

impl Variables {
    pub fn host() -> String {
        return std::env::var("HOST").unwrap_or("127.0.0.1".to_string());
    }
    pub fn port() -> u16 {
        return std::env::var("PORT")
            .unwrap_or("8080".to_string())
            .parse::<u16>()
            .unwrap_or(8080);
    }

    pub fn jwt_secret_key() -> String {
        return std::env::var("JWT_SECRET_KEY").unwrap_or("random_secret_key".to_string());
    }
}