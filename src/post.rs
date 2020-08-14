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

use serde::{ Serialize, Serializer };
use std::error::Error;
use rusqlite::Row;
use chrono::{ DateTime, NaiveDateTime, Utc };

#[derive(Serialize)]
pub struct Post {
    pub link: String,
    pub name: String,
    pub text: String,
    pub short_text: Option<String>,
    pub date: Option<PostDate>,
    pub lastmod: Option<PostDate>,
}

pub struct PostDate(pub DateTime<Utc>);

impl PostDate {
    pub fn from_timestamp(timestamp: i64) -> Option<PostDate> {
        if timestamp > 0 {
            Some(PostDate(DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp(timestamp, 0), Utc)
            ))
        } else {
            None
        }
    }
}

impl Serialize for PostDate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.format("%d.%m.%Y %H:%M").to_string())
    }
}

impl Post {
    pub fn from_row(row: &Row) -> Result<Post, Box<dyn Error>> {
        if row.column_count() == 7 {
            Ok(Post {
                link: row.get(0)?,
                name: row.get(1)?,
                text: row.get(2)?,
                short_text: row.get(3)?,
                date: PostDate::from_timestamp(row.get(4)?),
                lastmod: PostDate::from_timestamp(row.get(5)?),
            })
        } else if row.column_count() == 5 {
            Ok(Post {
                link: row.get(0)?,
                name: row.get(1)?,
                text: row.get(2)?,
                short_text: None,
                date: PostDate::from_timestamp(row.get(3)?),
                lastmod: PostDate::from_timestamp(row.get(4)?),
            })
        } else {
            Err("Invalid column count in the row".into())
        }
    }
}
