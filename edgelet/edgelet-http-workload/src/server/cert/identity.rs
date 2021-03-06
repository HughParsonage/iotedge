// Copyright (c) Microsoft. All rights reserved.

use edgelet_http::route::{BoxFuture, Handler, Parameters};
use futures::future;
use http::{Request, Response};
use hyper::{Body, Error as HyperError};

pub struct IdentityCertHandler;

impl Handler<Parameters> for IdentityCertHandler {
    fn handle(
        &self,
        _req: Request<Body>,
        _params: Parameters,
    ) -> BoxFuture<Response<Body>, HyperError> {
        let response = Response::new(Body::default());
        Box::new(future::ok(response))
    }
}
