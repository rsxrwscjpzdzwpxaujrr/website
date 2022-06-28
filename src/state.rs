/*
 * Copyright (c) 2020, 2022 Мира Странная <rsxrwscjpzdzwpxaujrr@yahoo.com>
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

use std::sync::{ Arc, RwLock };
use geoip2::{ Country, Reader };

use crate::config::Config;
use crate::auth::Auth;

pub struct State<'a> {
    pub tera: tera::Tera,
    pub conn: rusqlite::Connection,
    pub config: Arc<Config>,
    pub auth: RwLock<Auth>,
    pub geoip_reader: Option<Reader<'a, Country<'a>>>,
}
