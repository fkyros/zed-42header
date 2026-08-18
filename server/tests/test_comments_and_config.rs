use header42_lsp::comments::get_delimiters;
use header42_lsp::config::resolve_identity;

#[test]
fn test_c_delimiters() {
    let delim = get_delimiters("main.c");
    assert_eq!(delim.start, "/*");
    assert_eq!(delim.end, "*/");
    assert_eq!(delim.fill, '*');
    assert_eq!(delim.max_width, 80);

    let delim_cpp = get_delimiters("/path/to/test.cpp");
    assert_eq!(delim_cpp.start, "/*");
    assert_eq!(delim_cpp.end, "*/");
    assert_eq!(delim_cpp.fill, '*');
    assert_eq!(delim_cpp.max_width, 80);
}

#[test]
fn test_python_delimiters() {
    let delim = get_delimiters("script.py");
    assert_eq!(delim.start, "#");
    assert_eq!(delim.end, "#");
    assert_eq!(delim.fill, '*');
    assert_eq!(delim.max_width, 79);
}

#[test]
fn test_shell_and_makefile_delimiters() {
    let delim_sh = get_delimiters("deploy.sh");
    assert_eq!(delim_sh.start, "#");
    assert_eq!(delim_sh.end, "#");
    assert_eq!(delim_sh.fill, '*');
    assert_eq!(delim_sh.max_width, 80);

    let delim_mk = get_delimiters("Makefile");
    assert_eq!(delim_mk.start, "#");
    assert_eq!(delim_mk.end, "#");
    assert_eq!(delim_mk.fill, '*');
    assert_eq!(delim_mk.max_width, 80);
}

#[test]
fn test_js_ts_delimiters() {
    let delim_ts = get_delimiters("app.tsx");
    assert_eq!(delim_ts.start, "//");
    assert_eq!(delim_ts.end, "//");
    assert_eq!(delim_ts.fill, '*');
    assert_eq!(delim_ts.max_width, 80);
}

#[test]
fn test_html_xml_delimiters() {
    let delim_html = get_delimiters("index.html");
    assert_eq!(delim_html.start, "<!--");
    assert_eq!(delim_html.end, "-->");
    assert_eq!(delim_html.fill, '*');
    assert_eq!(delim_html.max_width, 80);
}

#[test]
fn test_lua_delimiters() {
    let delim_lua = get_delimiters("init.lua");
    assert_eq!(delim_lua.start, "--");
    assert_eq!(delim_lua.end, "--");
    assert_eq!(delim_lua.fill, '-');
    assert_eq!(delim_lua.max_width, 80);
}

#[test]
fn test_asm_delimiters() {
    let delim = get_delimiters("boot.s");
    assert_eq!(delim.start, ";");
    assert_eq!(delim.end, ";");
    assert_eq!(delim.fill, '*');
    assert_eq!(delim.max_width, 80);
}

#[test]
fn test_ocaml_delimiters() {
    let delim = get_delimiters("main.ml");
    assert_eq!(delim.start, "(*");
    assert_eq!(delim.end, "*)");
    assert_eq!(delim.fill, '*');
    assert_eq!(delim.max_width, 80);
}

#[test]
fn test_fortran_delimiters() {
    let delim = get_delimiters("calc.f90");
    assert_eq!(delim.start, "!");
    assert_eq!(delim.end, "!");
    assert_eq!(delim.fill, '/');
    assert_eq!(delim.max_width, 80);
}

#[test]
fn test_identity_resolution_settings_priority() {
    let (user, mail) = resolve_identity(Some("student42"), Some("student42@custom.org"));
    assert_eq!(user, "student42");
    assert_eq!(mail, "student42@custom.org");
}

#[test]
fn test_identity_resolution_user_provided_mail_fallback() {
    let (user, mail) = resolve_identity(Some("login42"), None);
    assert_eq!(user, "login42");
    // Should fallback to <user>@student.42.fr if no mail env var set or overridden
    assert!(mail.contains("login42") || mail.ends_with("@student.42.fr"));
}
