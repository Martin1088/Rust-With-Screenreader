use axum::response::Html;

pub async fn html_test() -> Html<&'static str> {
    Html(include_str!("../../kontakt.html"))
}
