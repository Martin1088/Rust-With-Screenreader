use axum::response::Html;

pub async fn test_html() -> Html<&'static str> {
    Html(include_str!("../../index.html"))
}
