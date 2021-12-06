# when

`when` is a small utility which tells you what time it is
somewhere or what some time is somewhere.

```
$ when "5pm in vienna -> san francisco"
time: 17:00:00
date: 2021-12-03
location: Vienna (Austria)
zone: Europe/Vienna (+0100)

time: 08:00:00
date: 2021-12-03
location: San Francisco (CA; United States)
zone: America/Los_Angeles (-0800)
```

## Examples

These are some other things you can do:

```
$ when "5pm in vienna -> london"
$ when "4pm on 17.05.2021 in vienna -> tokyo"
$ when "4pm yesterday in vienna -> vienna va"
$ when "in 4 hours in san francisco"
$ when "2pm in 2 days in new delhi
```

## Installation

Conveniently via cargo:

```
$ cargo install git+https://github.com/mitsuhiko/when
```

## Usage

Basically takes a single argument which is a string which describes the format
in roughly this syntax.  At the moment the first location is necessary but the
other location is optional.

```
time and date in location -> other location
```

Time and date can be provided roughly like this:

* `2:30pm`, `14:30`, `7:00`, `now`
* `14:30 tomorrow`
* `14:30`
* `17:00 on 20.05.` (DD.MM.)
* `17:00 on 20.05.2020` (DD.MM.YYYY)
* relative times (`in 4 hours`)

For locations many major cities are supported as well as
common timezone names like `Europe/Vienna`.
