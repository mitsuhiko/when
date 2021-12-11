# when

<img align="right" src="https://raw.githubusercontent.com/mitsuhiko/when/main/assets/hello.png" alt="example" width="480">

`when` is a small utility which tells you what time it is
somewhere or what some time is somewhere.  It takes human readable input like `now in vienna`
or `now in san francisco -> toronto` and figures out the answer.

It can be installed as a command line utility or [used online from the
browser](https://mitsuhiko.github.io/when/).

When takes a single argument which is a date and time expression.  In the most trivial
case it looks something like this:

```
$ when "now in vienna"
```

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

<br clear="left">

## Installation

Conveniently via cargo:

```
$ cargo install when-cli
```

There is also an [online version](https://mitsuhiko.github.io/when/) you can use
from your browser.

## Usage

Basically takes a single argument which is a string which describes the format
in roughly this syntax.  Both locations are optional.  The "local" location always
refers to the current machine's timezone.

```
time and date in location -> other location
```

Multiple locations can be suplied by using the arrow operator multiple times.  This
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