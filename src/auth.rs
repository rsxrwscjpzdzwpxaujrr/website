/*
 * Copyright (c) 2020, Мира Странная <rsxrwscjpzdzwpxaujrr@yahoo.com>
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

use actix_web::{ HttpRequest, HttpMessage, cookie::Cookie };
use std::error::Error;

pub struct Auth<'a> {
    token: String,
    cookie: Cookie<'a>,
}

impl Auth<'_> {
    pub fn new(token: String) -> Result<Auth<'static>, Box<dyn Error>> {
        Auth::check_token(token.as_str())?;

        Ok(Auth { token, cookie: Cookie::named("auth") })
    }

    pub fn has_admin_rights(&self, req: HttpRequest) -> bool {
        match req.cookie("auth") {
            Some(cookie) => { cookie.value() == self.token }
            _ => { false }
        }
    }

    pub fn auth(&mut self, token: String) -> bool {
        if token == self.token {
            self.cookie.set_value(token);
            return true;
        }

        return false;
    }

    pub fn cookie(&self) -> &Cookie {
        &self.cookie
    }

    fn check_token(token: &str) -> Result<(), Box<dyn Error>> {
        if !token.is_ascii() {
            return Err("Token should be ascii string".into());
        }

        if !token.len() < 32 {
            return Err("Token length should be over 32".into());
        }

        Ok(())
    }
}
