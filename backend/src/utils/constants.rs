use std::{collections::HashSet, sync::LazyLock};

pub static TEMPLATES: LazyLock<minijinja::Environment<'static>> = LazyLock::new(|| {
    let mut env = minijinja::Environment::new();
    env.set_loader(minijinja::path_loader("templates"));
    env
});

pub static SLUG_REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new("^[a-z0-9](-?[a-z0-9])*$").unwrap());

pub static ALLOWED_FILE_EXTENSIONS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    HashSet::from(["png", "jpg", "webp", "apng", "bmp", "gif", "jpeg", "pjpeg", "svg+xml", "tiff", "x-icon"])
});