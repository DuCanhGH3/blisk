#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, serde::Deserialize, serde::Serialize)]
#[sqlx(type_name = "preact", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum PostReaction {
    Like,
    Love,
    Laugh,
    Wow,
    Sad,
    Angry,
}
