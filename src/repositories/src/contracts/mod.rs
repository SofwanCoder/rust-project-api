use uuid::Uuid;

#[derive(Debug)]
pub struct CreateAuthRepositoryContract {
    pub user_id: Uuid,
    pub expires_at: chrono::NaiveDateTime,
}

#[derive(Debug)]
pub struct CreateUserRepositoryContract {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Default)]
pub struct UpdateUserRepositoryContract {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}
