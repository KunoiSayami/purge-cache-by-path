/*
 ** Copyright (C) 2021 KunoiSayami
 **
 ** This file is part of purge-cache-by-path and is released under
 ** the AGPL v3 License: https://www.gnu.org/licenses/agpl-3.0.txt
 **
 ** This program is free software: you can redistribute it and/or modify
 ** it under the terms of the GNU Affero General Public License as published by
 ** the Free Software Foundation, either version 3 of the License, or
 ** any later version.
 **
 ** This program is distributed in the hope that it will be useful,
 ** but WITHOUT ANY WARRANTY; without even the implied warranty of
 ** MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 ** GNU Affero General Public License for more details.
 **
 ** You should have received a copy of the GNU Affero General Public License
 ** along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use serde::{Serialize, Deserialize};
use anyhow::Result;
use std::collections::HashMap;
use reqwest::header::{HeaderValue, HeaderMap};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct FileStatus {
    status: String,
    path: String,
}

impl FileStatus {
    pub fn get_folder_name(&self) -> &str {
        if self.path.contains('/') {
            let mut iter = self.path.split('/');
            iter.next().unwrap()
        } else {
            "."
        }
    }
}

impl From<&String> for FileStatus {
    fn from(input: &String) -> Self {
        let mut iter = input.split_whitespace();
        Self {
            status: iter.next().unwrap().to_string(),
            path: iter.collect::<Vec<&str>>().join(""),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UrlFilePath {
    files: Vec<String>,
}

pub struct Requester {
    token: String,
    zone: String,
    urls: UrlFilePath,
}

impl Requester {
    pub fn new(token: &String, zone: &String, domain: &String, files: &Vec<String>) -> Self {
        let mut v: Vec<String> = Default::default();
        for file_status in files {
            let status = FileStatus::from(file_status);
            let folder_name = status.get_folder_name();
            if folder_name.eq(".") {
                continue
            }
            v.push(format!("{}/{}", domain, folder_name));
        }
        Self {
            token: token.clone(),
            zone: zone.clone(),
            urls: UrlFilePath { files: v },
        }
    }

    async fn send(&self) -> Result<()> {
        let mut map= HeaderMap::new();
        map.insert("Authorization", format!("Bearer {}", self.token).parse()?);
        let client = reqwest::Client::builder()
            .default_headers(map)
            .build()?;

        let response = client.post(format!("https://api.cloudflare.com/client/v4/zones/{}/purge_cache", self.zone))
            .json(&self.urls)
            .send()
            .await?;

        /*let j = response.json().await?;
        if ! j["success"] {
            log::error!("Got error: {:?}", j);
        }*/
        Ok(())
    }
}