use std::sync::LazyLock;

pub static TEMPLATES: LazyLock<minijinja::Environment<'static>> = LazyLock::new(|| {
    let mut env = minijinja::Environment::new();
    env.set_loader(minijinja::path_loader("templates"));
    env
});

pub const UPLOADS_DIRECTORY: &str = "uploads";
