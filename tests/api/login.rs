use crate::helpers::spawn_app;

#[tokio::test]
async fn an_error_flash_message_is_set_on_failure() {
    let app = spawn_app().await;

    let login_body = serde_json::json!({
        "username": "random-username",
        "password": "random-password",
    });
    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(response.headers().get("Location").unwrap(), "/login");

    let response = app.get_login().await;
    let html_page = response.text().await.unwrap();
    assert!(html_page.contains(r#"<p><i>Authentication failed</i></p>"#));

    let response = app.get_login().await;
    let html_page = response.text().await.unwrap();
    assert!(!html_page.contains(r#"<p><i>Authentication failed</i></p>"#));
}
