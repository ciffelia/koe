use regex::Regex;

// https://docs.rs/once_cell/latest/once_cell/#lazily-compiled-regex
macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}

pub fn url_regex() -> &'static Regex {
    regex!(r"https?://\S\S+")
}

pub fn custom_emoji_regex() -> &'static Regex {
    regex!(r"<(:\w+:)\d+>")
}
