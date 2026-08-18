use std::env;

/// Resolves the user login and email following the 4-tier resolution hierarchy:
/// 1. Zed `settings.json` initialization options
/// 2. 42 Environment Variables: `$USER42`, `$MAIL42`
/// 3. Standard Environment Variables: `$USER` (or `$USERNAME`), `$MAIL`
/// 4. Fallback: `<user>@student.42.fr` or `marvin` / `marvin@student.42.fr`
pub fn resolve_identity(
    settings_user: Option<&str>,
    settings_mail: Option<&str>,
) -> (String, String) {
    // 1. Resolve User
    let user = settings_user
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_string())
        .or_else(|| env::var("USER42").ok().filter(|s| !s.trim().is_empty()))
        .or_else(|| env::var("USER").ok().filter(|s| !s.trim().is_empty()))
        .or_else(|| env::var("USERNAME").ok().filter(|s| !s.trim().is_empty()))
        .unwrap_or_else(|| "marvin".to_string());

    // 2. Resolve Mail
    let mail = settings_mail
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_string())
        .or_else(|| env::var("MAIL42").ok().filter(|s| !s.trim().is_empty()))
        .or_else(|| env::var("MAIL").ok().filter(|s| !s.trim().is_empty()))
        .unwrap_or_else(|| format!("{}@student.42.fr", user));

    (user, mail)
}
