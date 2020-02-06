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

use std::error::Error;

use chrono::{ DateTime, NaiveDateTime, Utc };
use serde::Serialize;
use rusqlite::{ Row };

#[derive(Serialize)]
pub struct Url {
    pub loc: String,
    pub lastmod: String,
}

impl Url {
    pub fn from_row(row: &Row, host: String) -> Result<Url, Box<dyn Error>> {
        let link: String = row.get(0)?;
        let lastmod: i32 = row.get(1)?;

        Ok(Url::from_link(format!("/post/{}", link), host, lastmod))
    }

    pub fn from_link(link: String, host: String, lastmod: i32) -> Url {
        let date = if lastmod > 0 {
            DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp(lastmod as i64, 0), Utc
            )
            .format("%Y-%m-%dT%H:%M+00:00")
            .to_string()
        } else {
            String::from("")
        };

        Url {
            loc: format!("https://{}{}", host, link),
            lastmod: date,
        }
    }
}
