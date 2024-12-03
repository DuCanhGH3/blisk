use std::sync::LazyLock;

pub static TEMPLATES: LazyLock<minijinja::Environment<'static>> = LazyLock::new(|| {
    let mut env = minijinja::Environment::new();
    env.set_loader(minijinja::path_loader("templates"));
    env
});

pub static SLUG_REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new("^[a-z0-9](-?[a-z0-9])*$").unwrap());
