pub fn sanitize_response(text: &str) -> String {
    format!("`{}`", text.replace('`', ""))
}
