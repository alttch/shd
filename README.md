# Show pretty HDD/SSD list

Long time ago when I've switched from Solaris to Linux, I missed "hd" utility.
In Linux "hd" command is used for a hex dump, in Solaris it displayed a pretty
table with HDD info.

I wrote "shd" shell script with a similar functionality. Now I rewrote it in
Python, added options and pretty colors, and releasing it to public.

<img src="https://raw.githubusercontent.com/alttch/shd/master/demo.gif" />

## Installation

Install *smartmontools*, then:

```
python3 -m pip install shd
```

## Usage

Tool should be started under root, to obtain S.M.A.R.T. info

```
shd [-h] [--temp-warn TEMP] [--temp-crit TEMP] [-R] [-y] [-e] [-s] [-J]

  --temp-warn TEMP  Warning temperature, default: 40 C
  --temp-crit TEMP  Critical temperature, default: 45 C
  --fh, --fahrenheit  Temperature in Fahrenheit
  -R, --raw         Suppress colors
  -y, --full        Display full disk info
  -e, --errors      Display only disks with errors / critical temperature
  -s, --no-header   Suppress header
  -J, --json        Output as JSON
```

## Exit codes

* **1** critical temperature
* **2** errors detected

The tool considers drive has an errors, if the following S.M.A.R.T. fields are
greater than zero:

* End-to-End_Error
* Seek_Error_Rate
* Raw_Read_Error_Rate
* UDMA_CRC_Error_Count
