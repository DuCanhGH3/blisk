#[derive(Default, sqlx::FromRow)]
#[sqlx(default)]
pub struct User {
    pub id: Option<i32>,
    pub email: Option<String>,
    pub name: Option<String>,
    pub is_verified: Option<bool>,
    pub role: Option<String>,
    pub password: Option<String>,
}
