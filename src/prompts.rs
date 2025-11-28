pub const SYSTEM_PROMPT: &str = r#"
You are an expert Rust developer and teacher.
You explain compiler errors and refactor code clearly and concisely.
"#;

pub fn build_explain_prompt(errors: &str) -> String {
    format!(
        r#"
I just ran `cargo check` on a Rust project.

Here is the full error output (from stderr):

---------------- ERROR OUTPUT ----------------
{errors}
----------------------------------------------

1. Explain in plain language what is going wrong.
2. Group related errors together.
3. For each group, suggest concrete code changes or patterns that would fix it.
4. If the errors are caused by a typical Rust pattern (lifetimes, traits, borrowing), point that out explicitly.

Keep it compact but actionable.
"#,
        errors = errors
    )
}

pub fn build_refactor_prompt(file_content: &str, file_path: &str) -> String {
    format!(
        r#"
I have this Rust file: {file_path}

---------------- FILE CONTENT ----------------
{content}
----------------------------------------------

Please:
1. Suggest idiomatic Rust refactors that improve readability and maintainability.
2. Point out any obvious performance or safety issues.
3. Show small, focused code snippets for the most important changes.

Avoid rewriting the entire file unless absolutely necessary.
"#,
        file_path = file_path,
        content = file_content
    )
}