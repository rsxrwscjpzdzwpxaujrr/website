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

use chrono::{ DateTime, Utc };
use serde:: { Serializer, Serialize };
use rusqlite::{ Row };

use crate::post::PostDate;

#[derive(Serialize)]
pub struct Url {
    pub loc: String,
    pub lastmod: Option<SitemapDate>,
}

pub struct SitemapDate(DateTime<Utc>);

impl SitemapDate {
    pub fn from_timestamp(timestamp: i64) -> Option<SitemapDate> {
        Some(SitemapDate(PostDate::from_timestamp(timestamp)?.0))
    }
}

impl Serialize for SitemapDate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(&self.0.format("%Y-%m-%dT%H:%M+00:00").to_string())
    }
}

impl Url {
    pub fn from_row(row: &Row, host: String) -> Result<Url, Box<dyn Error>> {
        let link: String = row.get(0)?;
        let date: i64 = row.get(1)?;
        let mut lastmod: i64 = row.get(2)?;

        if lastmod == 0 {
            lastmod = date;
        }

        Ok(Url::from_link(format!("/articles/{}", link), host, lastmod))
    }

    pub fn from_link(link: String, host: String, lastmod: i64) -> Url {
        Url {
            loc: format!("https://{}{}", host, link),
            lastmod: SitemapDate::from_timestamp(lastmod),
        }
    }
}
