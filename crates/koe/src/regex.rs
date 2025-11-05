use regex::Regex;

macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: std::sync::LazyLock<regex::Regex> =
            std::sync::LazyLock::new(|| regex::Regex::new($re).unwrap());
        &RE
    }};
}

pub fn url_regex() -> &'static Regex {
    regex!(r"https?://\S\S+")
}

pub fn custom_emoji_regex() -> &'static Regex {
    regex!(r"<(:\w+:)\d+>")
}
