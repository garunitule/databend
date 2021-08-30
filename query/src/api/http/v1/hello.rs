// Copyright 2020 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use axum::extract::Extension;
use common_runtime::tokio;

use crate::configs::Config;


pub async fn hello_handler(cfg: Extension<Config>) -> String {
    format!("{:?}", cfg)
}

#[tokio::test]
async fn test_hello() -> common_exception::Result<()> {
    use axum::body::Body;
    use axum::handler::get;
    use axum::http::Request;
    use axum::http::StatusCode;
    use axum::http::{self};
    use axum::AddExtensionLayer;
    use axum::Router;
    use pretty_assertions::assert_eq;
    use tower::ServiceExt;
    use crate::api::http::v1::config::config_handler;
    use crate::configs::Config; // for `app.oneshot()`

    let conf = Config::default();
    let cluster_router = Router::new()
        .route("/v1/hello", get(hello_handler))
        .route("/v1/config", get(config_handler))
        .layer(AddExtensionLayer::new(conf.clone()));
    // health check
    {
        let response = cluster_router
            .clone()
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/v1/hello")
                    .method(http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
    // health check(config)
    {
        let response = cluster_router
            .clone()
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/v1/config")
                    .method(http::Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
    Ok(())
}
