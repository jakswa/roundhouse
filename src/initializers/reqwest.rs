use axum::Router as AxumRouter;
use loco_rs::prelude::*;

pub struct ReqwestClientInitializer;

#[async_trait]
impl Initializer for ReqwestClientInitializer {
    fn name(&self) -> String {
        "reqwest_client".to_string()
    }

    async fn after_routes(&self, router: AxumRouter, _ctx: &AppContext) -> Result<AxumRouter> {
        let scary_client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .pool_max_idle_per_host(1)
            .build()
            .unwrap();
        Ok(router.layer(axum::Extension(scary_client)))
    }
}
