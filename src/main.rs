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

use clap::Clap;
use std::process;
#[derive(Clap)]
struct Opts {
    #[cfg(not(windows))]
    #[clap(long, default_value = "/usr/bin/git")]
    git_bin_path: String,


    #[cfg(windows)]
    #[clap(long, default_value = "C:\\Program Files\\Git\\mingw64\\bin\\git.exe")]
    git_bin_path: String,

    #[clap(long)]
    token: String,

    #[clap(long)]
    zone: String,

    #[clap(long)]
    domain: String,
}

fn main() {
    let opts: Opts = Opts::parse();

    let git_output = process::Command::new(opts.git_bin_path.clone())
        .arg("diff")
        .arg("--name-status")
        .arg("HEAD^")
        .output()
        .unwrap()
        .stdout;
    let output_string = String::from_utf8(git_output).unwrap();
    requester::Requester::new(&opts.git_bin_path, &opts.token, &opts.domain,&output_string.lines().map(|s| s.to_string()).collect());
    println!("Bin path: {}, Token: {}, Output: {}", opts.git_bin_path, opts.token, output_string);
}
