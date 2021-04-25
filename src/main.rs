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
mod requester;
mod configure;

use clap::{App, Arg};
use std::process;
use anyhow::Result;
use configure::DEFAULT_GIT_BIN_PATH;
use std::path::Path;

fn main() -> Result<()> {
    env_logger::init();
    //let opts: Opts = Opts::parse();

    let arg_matches = App::new(env!("CARGO_PKG_NAME"))
        .arg(Arg::new("token")
            .takes_value(true)
            .long("token")
            .require_equals(true))
        .arg(Arg::new("domain")
            .long("domain")
            .takes_value(true)
            .require_equals(true))
        .arg(Arg::new("zone")
            .long("zone")
            .takes_value(true)
            .require_equals(true))
        .arg(Arg::new("git_bin_path")
            .takes_value(true)
            .long("git_bin")
            .default_value(DEFAULT_GIT_BIN_PATH))
        .arg(Arg::new("cfg")
            .aliases(&["config", "configure"])
            //.exclusive(true)
            .conflicts_with_all(&["token", "domain", "zone"])
            .takes_value(true))
        .arg(Arg::new("dry_run")
            .long("dry-run")
            .aliases(&["test", "dry", "dr"]))
        .get_matches();

    let config = if let Some(cfg_path) = arg_matches.value_of("cfg") {
        let path = Path::new(cfg_path);
        let context = std::fs::read_to_string(path)?;
        toml::from_str(context.as_str())?
    } else {
        configure::Configure::from(&arg_matches)
    };

    let git_output = process::Command::new(config.get_git_bin())
        .arg("diff")
        .arg("--name-status")
        .arg("HEAD^")
        .output()
        .unwrap()
        .stdout;
    let output_string = String::from_utf8(git_output).unwrap();
    let cf_requester = config.to_requester(&output_string.lines().map(|s| s.to_string()).collect());
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(cf_requester.send(arg_matches.is_present("dry_run")))?;
    println!("Bin path: {}, Token: {}, Output: {}", config.get_git_bin(), config.get_token(), output_string);
    Ok(())
}
