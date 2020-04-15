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

use serde::Serialize;
use std::error::Error;
use rusqlite::Row;
use chrono::{ DateTime, NaiveDateTime, Utc };

#[derive(Serialize)]
pub struct Post {
    pub link: String,
    pub name: String,
    pub text: String,
    pub short_text: Option<String>,
    pub date: String,
}

impl Post {
    pub fn from_row(row: &Row) -> Result<Post, Box<dyn Error>> {
        let timestamp = row.get(4)?;

        let date = if timestamp > 0 {
            DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp(timestamp, 0), Utc
            )
            .format("%d.%m.%Y %H:%M UTC")
            .to_string()
        } else {
            String::from("")
        };

        Ok(Post {
            link: row.get(0)?,
            name: row.get(1)?,
            text: row.get(2)?,
            short_text: row.get(3)?,
            date: date,
        })
    }
}
