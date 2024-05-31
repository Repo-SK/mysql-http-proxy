use ntex::service::{Middleware, Service, ServiceCtx};
use ntex::web;

pub struct AuthMiddleware {
    bearer_token: String
}

impl AuthMiddleware {
    pub fn new(bearer_token: String) -> Self {
        AuthMiddleware { bearer_token }
    }
}

impl<S> Middleware<S> for AuthMiddleware {
    type Service = AuthMiddlewareService<S>;

    fn create(&self, service: S) -> Self::Service {
        AuthMiddlewareService {
            service,
            bearer_token: self.bearer_token.clone()
        }
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
    bearer_token: String
}

impl<S, Err> Service<web::WebRequest<Err>> for AuthMiddlewareService<S>
    where
        S: Service<web::WebRequest<Err>, Response = web::WebResponse, Error = web::Error>,
        Err: web::ErrorRenderer,
{
    type Response = web::WebResponse;
    type Error = web::Error;

    ntex::forward_ready!(service);

    async fn call(&self, req: web::WebRequest<Err>, ctx: ServiceCtx<'_, Self>) -> Result<Self::Response, Self::Error> {
        if let Some(auth_header) = req.headers().get("Authorization") {
            if let Ok(auth_value) = auth_header.to_str() {
                if auth_value == format!("Bearer {}", self.bearer_token) {
                    return ctx.call(&self.service, req).await;
                }
            }
        }

        return Err(web::error::ErrorUnauthorized("Unauthorized").into());
    }
}