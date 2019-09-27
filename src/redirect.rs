/*
 * Copyright (c) 2019, Мира Странная <rsxrwscjpzdzwpxaujrr@yahoo.com>
 *
 * This program is free software: you can redistribute it and/or
 * modify it under the terms of the GNU Affero General Public License
 * as published by the Free Software Foundation, either version 3 of
 * the License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::rc::Rc;

use actix_service::{ Service, Transform };
use actix_web::{ HttpResponse, dev::ServiceRequest, dev::ServiceResponse, Error };
use futures::future::{ ok, FutureResult };
use futures::Poll;

pub struct Redirect(pub Rc<String>);

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
    host: Rc<String>,
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
