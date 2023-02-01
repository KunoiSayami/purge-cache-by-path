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
mod configure;
mod requester;

use anyhow::Result;
use clap::{arg, Arg, Command};
use configure::DEFAULT_GIT_BIN_PATH;
use std::io::Write;
use std::path::Path;
use std::process;

fn main() -> Result<()> {
    let arg_matches = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .args(&[
            arg!(<token> "CloudFlare api token"),
            arg!(<domain> "Your website domain"),
            arg!(<zone> "Your domain zone ID"),
            Arg::new("git_bin_path")
                .long("git_bin")
                .default_value(DEFAULT_GIT_BIN_PATH),
            arg!(--systemd "Pass this argument to disable timestamp in log output"),
            Arg::new("dry_run")
                .long("dry-run")
                .help("Run without send any request to cloudflare api server")
                .aliases(&["test", "dry", "dr"]),
            Arg::new("cfg")
                .aliases(&["config", "configure"])
                //.exclusive(true)
                .help("Specify configure file without passing arguments from command line")
                .require_equals(true)
                .conflicts_with_all(&["token", "domain", "zone"]),
        ])
        .get_matches();

    if arg_matches.get_flag("with-systemd") {
        env_logger::Builder::from_default_env()
            .format(|buf, record| writeln!(buf, "[{}] - {}", record.level(), record.args()))
            .init()
    } else {
        env_logger::init();
    }

    let config = if let Some(cfg_path) = arg_matches.get_one::<String>("cfg") {
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
        .block_on(cf_requester.send(arg_matches.get_flag("dry_run")))?;
    Ok(())
}
