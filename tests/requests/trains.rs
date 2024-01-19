use loco_rs::testing;
use roundhouse::app::App;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn can_request_root() {
    testing::request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/").await;
        assert_eq!(res.status_code(), 200);
    })
    .await;
}
