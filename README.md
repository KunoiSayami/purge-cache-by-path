# Hugo purge CloudFlare cache by path

## Configure

Configure file format should like this below:

```toml
token = "<token here>"
zone = "<zone here>"
domain = "example.com"
```

Then, run program with configure file name

```shell
purge-cache purge.toml
```

Or you can pass arguments from command line

```shell
purge-cache --token "token" --zone "zone" --domain "domain"
```

You should run this program under your website root folder, it uses `git` command to fetch file changes.

To view logs, you should set [`RUST_LOG`](https://docs.rs/env_logger/0.8.3/env_logger/#example) environment variable.

## Arguments

```
Usage: purge-cache-by-path [OPTIONS] <token> <domain> <zone> [cfg]

Arguments:
  <token>   CloudFlare api token
  <domain>  Your website domain
  <zone>    Your domain zone ID
  [cfg]     Specify configure file without passing arguments from command line

Options:
      --git_bin <git_bin_path>  [default: /usr/bin/git]
      --systemd                 Pass this argument to disable timestamp in log output
      --dry-run <dry_run>       Run without send any request to cloudflare api server
  -h, --help                    Print help
  -V, --version                 Print version

```

## License

[![](https://www.gnu.org/graphics/agplv3-155x51.png)](https://www.gnu.org/licenses/agpl-3.0.txt)

Copyright (C) 2021-2023 KunoiSayami

This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
