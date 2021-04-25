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

use anyhow::Result;
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};

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

    pub fn get_path_vec(&self) -> Vec<&str> {
        if self.path.contains('/') {
            self.path.split('/').collect()
        } else {
            vec![".", self.path.as_str()]
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

impl UrlFilePath {
    pub fn len(&self) -> usize {
        self.files.len()
    }
}

pub struct Requester {
    token: String,
    zone: String,
    urls: UrlFilePath,
    purge_all: bool,
}

impl Requester {
    pub fn new(token: &str, zone: &str, domain: &str, files: &Vec<String>) -> Self {
        let mut v: Vec<String> = Default::default();
        let mut purge_all = false;
        for file_status in files {
            let status = FileStatus::from(file_status);
            let mut folder_name = status.get_folder_name();

            if folder_name.starts_with('.')
                || vec!["archetypes"].into_iter().any(|x| folder_name.eq(x))
            {
                continue;
            }
            let folder_vec = status.get_path_vec();
            match folder_name {
                "content" | "static" => {
                    if folder_vec.len() > 1 {
                        folder_name = folder_vec[1]
                    }
                }
                &_ => {
                    purge_all = true;
                    break;
                }
            }

            v.push(format!("{}/{}", domain, folder_name));
        }
        Self {
            token: token.to_string(),
            zone: zone.to_string(),
            urls: UrlFilePath { files: v },
            purge_all,
        }
    }

    pub async fn send(&self, dry_run: bool) -> Result<()> {
        if !self.purge_all && self.urls.len() == 0 {
            log::info!("There is nothing should purge");
            return Ok(());
        }
        let mut map = HeaderMap::new();
        map.insert("Authorization", format!("Bearer {}", self.token).parse()?);
        map.insert("Content-Type", "application/json".parse()?);
        let client = reqwest::Client::builder().default_headers(map).build()?;

        let post_text = if self.purge_all {
            r#"{"purge_everything":true}"#.to_string()
        } else {
            serde_json::to_string(&self.urls).unwrap()
        };
        if dry_run {
            log::info!("Dry run: {}", post_text);
            return Ok(());
        }
        let response = client
            .post(format!(
                "https://api.cloudflare.com/client/v4/zones/{}/purge_cache",
                self.zone
            ))
            .body(post_text)
            .send()
            .await?;
        if !response.status().is_success() {
            log::error!("Got error while request purge cache");
            log::error!("Raw body: {}", response.text().await?);
        }
        Ok(())
    }
}
