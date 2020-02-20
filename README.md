# Show pretty HDD/SSD list

Long time ago when I've switched from Solaris to Linux, I missed "hd" utility.
In Linux "hd" command is used for a hex dump, in Solaris it displayed a pretty
table with HDD info.

I wrote "shd" shell script with a similar functionality. Now I've rewrote it in
Python, added options and pretty colors, and releasing it to public.

<img src="https://raw.githubusercontent.com/alttch/shd/master/demo.gif" />

## Installation

Install *smartmontools*, then:

```
python3 -m pip install shd
```

## Usage

Script should be run as root, to obtain S.M.A.R.T. info

```
shd [-h] [--temp-warn TEMP] [--temp-crit TEMP] [-R] [-y] [-e] [-s] [-J]

  --temp-warn TEMP  Warning temperature (C), default: 40
  --temp-crit TEMP  Critical temperature (C), default: 45
  -R, --raw         Suppress colors
  -y, --full        Display full disk info
  -e, --errors      Display only disks with errors / critical temperature
  -s, --no-header   Suppress header
  -J, --json        Output as JSON
```

p.s. if someone needs temperature in Fahrenheit, feel free to implement or drop
me an issue.
