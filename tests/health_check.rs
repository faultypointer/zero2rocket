#[rocket::async_test]
async fn health_check_works() {
    let server = zero2rocket::run();
    let client = rocket::local::asynchronous::Client::tracked(server)
        .await
        .expect("Failed to build rocket client.");

    let response = client.get("/health_check").dispatch().await;

    assert!(response.status().class().is_success());
    assert!(response.body().is_none())
}
