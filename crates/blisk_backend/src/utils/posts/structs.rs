#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, serde::Deserialize, serde::Serialize)]
#[sqlx(type_name = "breact", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Reaction {
    Like,
    Dislike,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub author_name: String,
    pub reaction: Reaction,
}
