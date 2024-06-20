#[derive(serde::Serialize, serde::Deserialize)]
pub struct Comment {
    pub id: i64,
    pub content: String,
    pub author_name: String,
    pub replies: Option<sqlx::types::Json<Vec<Comment>>>,
}
