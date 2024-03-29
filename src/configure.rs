/*
 ** Copyright (C) 2021-2023 KunoiSayami
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
use crate::requester;
use clap::ArgMatches;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

#[cfg(windows)]
pub const DEFAULT_GIT_BIN_PATH: &str = "C:\\Program Files\\Git\\mingw64\\bin\\git.exe";

#[cfg(not(windows))]
pub const DEFAULT_GIT_BIN_PATH: &str = "/usr/bin/git";

#[derive(Serialize, Deserialize, Clone, Debug)]
struct GitBinString(String);

impl GitBinString {
    fn get_path(&self) -> &String {
        &self.0
    }
}

impl Default for GitBinString {
    fn default() -> Self {
        Self(DEFAULT_GIT_BIN_PATH.to_string())
    }
}

impl From<&String> for GitBinString {
    fn from(s: &String) -> Self {
        Self { 0: s.clone() }
    }
}

impl std::fmt::Display for GitBinString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Configure {
    #[serde(default)]
    git_bin: GitBinString,
    token: String,
    zone: String,
    domain: String,
}

impl Configure {
    pub fn to_requester(&self, files: &Vec<String>) -> requester::Requester {
        requester::Requester::new(self.get_token(), self.get_zone(), self.get_domain(), files)
    }

    pub fn get_git_bin(&self) -> &String {
        self.git_bin.get_path()
    }

    pub fn get_token(&self) -> &String {
        &self.token
    }

    pub fn get_zone(&self) -> &String {
        &self.zone
    }

    pub fn get_domain(&self) -> &String {
        &self.domain
    }
}

impl From<&ArgMatches> for Configure {
    fn from(matches: &ArgMatches) -> Self {
        let git_bin_path: &String = matches.get_one("git_bin_path").unwrap();
        let zone: &String = matches.get_one("zone").unwrap();
        let domain: &String = matches.get_one("domain").unwrap();
        let token: &String = matches.get_one("token").unwrap();
        Configure {
            git_bin: GitBinString::from(git_bin_path),
            token: token.to_string(),
            zone: zone.to_string(),
            domain: domain.to_string(),
        }
    }
}
