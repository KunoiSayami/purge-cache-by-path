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
use crate::requester;
use clap::ArgMatches;

#[cfg(windows)]
pub const DEFAULT_GIT_BIN_PATH: &str = "C:\\Program Files\\Git\\mingw64\\bin\\git.exe";

#[cfg(not(windows))]
pub const DEFAULT_GIT_BIN_PATH: &str = "/usr/bin/git";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Configure {
    git_bin: String,
    token: String,
    zone: String,
    domain: String
}

impl Configure {
    pub fn to_requester(&self, files: &Vec<String>) -> requester::Requester {
        requester::Requester::new(self.get_token(), self.get_zone(), self.get_domain(), files)
    }

    pub fn get_git_bin(&self) -> &String {
        &self.git_bin
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

impl From<&clap::ArgMatches> for Configure {
    fn from(matches: &ArgMatches) -> Self {
        let git_bin_path = matches.value_of("git_bin_path").unwrap();
        let zone = matches.value_of("zone").unwrap();
        let domain = matches.value_of("domain").unwrap();
        let token = matches.value_of("token").unwrap();
        Configure {
            git_bin: git_bin_path.to_string(),
            token: token.to_string(),
            zone: zone.to_string(),
            domain: domain.to_string()
        }
    }
}