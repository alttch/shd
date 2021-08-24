# Show pretty HDD/SSD list

Long time ago when I've switched from Solaris to Linux, I missed the "hd"
utility. In Linux "hd" command is used for hex dump, in Solaris it displayed a
pretty table with HDD info.

I had written "shd" shell script with a similar functionality. After I rewrote
it in Python, added options and pretty colors. The current version 0.1 comes in
Rust, as statically built binaries for x86\_64 Linux, i686, ARM and AARCH64.

<img src="https://raw.githubusercontent.com/alttch/shd/master/demo.gif" />

## Installation

Install *smartmontools (>=7.0)*, then download the appropriate binary from the
[releases](https://github.com/alttch/shd/releases) page, chmod it to 755 and
enjoy.

## Usage

Tool should be started under root, to obtain S.M.A.R.T. info

```
shd [-h] [--temp-warn TEMP] [--temp-crit TEMP] [-R] [-y] [-e] [-s] [-J]

  --temp-warn TEMP  Warning temperature, default: 40 C
  --temp-crit TEMP  Critical temperature, default: 45 C
  --f, --fahrenheit Temperature in Fahrenheit
  -R, --raw         Suppress colors
  -y, --full        Display full disk info
  -e, --errors      Display only disks with errors / critical temperature
  -s, --no-header   Suppress header
```

## Exit codes

* **1** critical temperature
* **2** errors detected
* **3** smartctl error

The tool considers a drive has errors if its smart status is either not
reported or reported as "false".
