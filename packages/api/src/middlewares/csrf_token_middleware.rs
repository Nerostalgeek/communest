use actix_service::{Service, Transform};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    error, Error, HttpResponse,
};
use futures::future::{ok, Ready};
use std::task::{Context, Poll};

use crate::utils::csrf::validate_csrf_token;

pub struct CsrfMiddleware<S> {
    service: S,
}

impl<S, B> Transform<S, ServiceRequest> for CsrfMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static + Send, // Enforce B is Send
    S::Future: Send,   // Ensure the future is Send
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = CsrfMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(CsrfMiddlewareService { service })
    }
}

pub struct CsrfMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for CsrfMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static + Send, // Ensure B is Send
    S::Future: Send,   // Ensure the future is Send
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = futures::future::BoxFuture<'static, Result<Self::Response, Self::Error>>;
    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }
    fn call(&self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);

        // Extract necessary information from `req` before the async block.
        // For example, clone the CSRF token from the headers or cookies.
        let csrf_token = req
            .headers()
            .get("X-CSRF-Token")
            .map(|value| value.to_str().unwrap_or_default().to_owned());

        Box::pin(async move {
            // Now `csrf_token` is a cloned String, which is `Send`, so it's safe to use here.
            if let Some(token) = csrf_token {
                if validate_csrf_token(&token).await {
                    // Proceed with the future as before
                    return fut.await;
                }
            }

            // If the token is invalid or not present, construct an error response
            let err_response = HttpResponse::Forbidden().finish().into_body();
            let error = actix_web::error::ErrorForbidden("CSRF token validation failed");
            Err(error)
        })
    }
}
