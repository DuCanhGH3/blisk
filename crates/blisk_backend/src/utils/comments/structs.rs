#[derive(serde::Serialize, serde::Deserialize)]
pub struct Comment {
    id: i64,
    path: String,
    content: String,
    author_name: String,
    level: i32,
    post_id: i64,
    replies: Vec<Comment>,
}
