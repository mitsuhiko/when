<img align="right" src="https://raw.githubusercontent.com/mitsuhiko/when/main/assets/hello.png" alt="example" width="460">

<div align="left">
  <h3><em>when:</em> a timezone utility for the command line</h3>
</div>

[![Build Status](https://github.com/mitsuhiko/when/workflows/Tests/badge.svg?branch=main)](https://github.com/mitsuhiko/when/actions?query=workflow%3ATests)
[![Crates.io](https://img.shields.io/crates/d/when-cli.svg)](https://crates.io/crates/when-cli)
[![License](https://img.shields.io/github/license/mitsuhiko/when)](https://github.com/mitsuhiko/when/blob/main/LICENSE)

```
$ when "now in vienna"
```

`when` is a small utility which tells you what time it is somewhere or what some time is somewhere.
You can use it from the command line or [uses it online from the browser](https://mitsuhiko.github.io/when/).

**These are some input examples**:

* `now`
* `2 hours ago in yyz`
* `5pm in yyz -> sfo`
* `5pm in vienna -> london`
* `4pm on 17.05.2021 in vienna -> tokyo`
* `4pm yesterday in vienna -> vienna va`
* `in 4 hours in san francisco`
* `2pm in 2 days in new delhi`
* `now in yyz -> sfo -> vie -> lhr`
* `unix 1639067620 in tokyo`

## Installation

Conveniently via cargo:

```
$ cargo install when-cli
```

There is also an [online version](https://mitsuhiko.github.io/when/) you can use
from your browser.

Note that this project requires a Rust 2021 compatible compiler (1.56.0 or
later).  Attempting to install this package on an older compiler will result
in compilation errors (``feature `edition2021` is required``).  If you're
using rustup make sure to update (`rustup update`), you might be on an older
version.

## Usage

Basically takes a single argument which is a string which describes the format
in roughly this syntax.  Both locations are optional.  The "local" location always
refers to the current machine's timezone.

```
time and date in location -> other location
```

Multiple locations can be supplied by using the arrow operator multiple times.  This
means you can do things like `now in yyz -> sfo -> vie`.

Time and date can be provided roughly like this:

* `2:30pm`, `14:30`, `7:00`, `now`
* `14:30 tomorrow`
* `14:30`
* `17:00 on 20.05.` (DD.MM.)
* `17:00 on 20.05.2020` (DD.MM.YYYY)
* relative times (`in 4 hours` or `4 hours ago`)
* unix timestamps (`unix:TS` or `unix TS`)

For locations many major cities are supported as well as common timezone names
like `Europe/Vienna`.  A certain amount of disambiugation is possible with city
names.  For instance `Vienna VA` (Virginia) is different than `Vienna AT`
(Austria).
