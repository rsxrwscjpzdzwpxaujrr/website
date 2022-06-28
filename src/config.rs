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

use std::{ fs, io::BufReader, error::Error };
use serde::Deserialize;
use serde_json::from_reader;

#[derive(Deserialize)]
pub struct Config {
    pub priv_key_file: String,
    pub cert_chain_file: String,
    pub host: String,
    pub database: String,
    pub templates: String,
    pub token: String,
    pub geoip_db_file: String
}

impl Config {
    pub fn read_from_file(path: &str) -> Result<Config, Box<dyn Error>> {
        let buf = BufReader::new(fs::File::open(path)?);
        let config = from_reader(buf)?;

        Ok(config)
    }
}
