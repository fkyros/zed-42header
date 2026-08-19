/// Resolves the user login and email strictly from Zed initialization options
/// and safe fallbacks (no external environment access):
/// 1. Zed `settings.json` initialization options (`user` and `mail`)
/// 2. Default Fallback: `marvin` / `<user>@student.42.fr`
pub fn resolve_identity(
    settings_user: Option<&str>,
    settings_mail: Option<&str>,
) -> (String, String) {
    let user = settings_user
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "marvin".to_string());

    let mail = settings_mail
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_string())
        .unwrap_or_else(|| format!("{}@student.42.fr", user));

    (user, mail)
}
