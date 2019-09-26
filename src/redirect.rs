use actix_service::{ Service, Transform };
use actix_web::{ HttpResponse, dev::ServiceRequest, dev::ServiceResponse, Error };
use futures::future::{ ok, FutureResult };
use futures::Poll;

pub struct Redirect(String);

impl Redirect {
    pub fn new(host: String) -> Redirect {
        Redirect(host)
    }
}

impl<S, B> Transform<S> for Redirect
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RedirectMiddleware<S>;
    type Future = FutureResult<Self::Transform, Self::InitError>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RedirectMiddleware { service, host: self.0.clone() })
    }
}

pub struct RedirectMiddleware<S> {
    service: S,
    host: String,
}

impl<S, B> Service for RedirectMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = FutureResult<Self::Response, Self::Error>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.service.poll_ready()
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let uri_parts: actix_web::http::uri::Parts = req.uri().to_owned().into_parts();

        ok(req.into_response(HttpResponse::PermanentRedirect()
            .header("Location",
                format!("https://{}{}",
                    self.host,
                    uri_parts.path_and_query.unwrap().as_str()
            ))
            .finish()
            .into_body()
        ))
    }
}
