use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpResponse};
use futures::future::{ok, Ready};
use futures::Future;
use std::env;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct ErrorHandling;

impl<S, B> Transform<S> for ErrorHandling
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = ErrorHandlingMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(ErrorHandlingMiddleware { service })
    }
}

pub struct ErrorHandlingMiddleware<S> {
    service: S,
}

impl<S, B> Service for ErrorHandlingMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let is_dev = env::var("RUST_ENV").unwrap_or_else(|_| "production".into()) == "development";

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            if is_dev {
                // Modifier la réponse en cas d'erreur pour le développement
            } else {
                // Utiliser une réponse d'erreur générique pour la production
            }

            Ok(res)
        })
    }
}
