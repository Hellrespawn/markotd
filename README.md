# MarkOTD

A themeable MOTD for home servers.

## Installation

Clone the repository and run `make install` will install to `~/.bin`. Requires Rust.

## Use

```txt
Usage: markotd [TEMPLATE]

Arguments:
  [TEMPLATE]  [default: json]

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Configuration

The application is configured via environment variables:

- `NOTIFY_UPDATE_HOURS` - The number of hours after which you'll see the last updated time.

- `DUR_DIV` - Maximum amount of time to divide seconds in duration, e.g. 0 shows seconds, 1 shows minutes and seconds, 2 show hours, minutes and seconds, etc.

- `DF_WHITELIST` and `DF_BLACKLIST` - Regular expressions which determine which drives from the output of `df` are shown. By default it only shows drive that start with "/dev" and "[A-Z]:". Blacklist takes priority over whitelist.

The context available to templates can be found in `src/template.rs::MotdContext`.

## TODO

- TODO Support color in templates
- TODO? Use `humantime` to format durations?
- TODO? "Syntax Highlighting"?
- TODO? System load?
- TODO? Hint at number of ignored drives?
