use clap::Clap;
use colored::control::SHOULD_COLORIZE;
use colored::{ColoredString, Colorize};
use glob::glob;
use serde::Deserialize;
use std::io::{self, Write};
use std::process::{exit, Command};

#[macro_use]
extern crate prettytable;

const TEMP_WARN_DEFAULT_C: f32 = 40.0;
const TEMP_CRIT_DEFAULT_C: f32 = 45.0;

const TEMP_WARN_DEFAULT_C_NVME: f32 = 50.0;
const TEMP_CRIT_DEFAULT_C_NVME: f32 = 60.0;

const VERSION: &str = "0.1.2";

const EXIT_CODE_NORMAL: i32 = 0;
const EXIT_CODE_TEMP: i32 = 1;
const EXIT_CODE_ERRORS: i32 = 2;
const EXIT_CODE_SMARTCTL: i32 = 3;

macro_rules! empty {
    () => {
        String::new()
    };
}

macro_rules! empty_c {
    () => {
        String::new().normal()
    };
}

macro_rules! s {
    ($s: expr) => {
        $s.map_or(empty!(), |v| format!("{}", v))
    };
}

trait Fahrenheit {
    fn to_fahrenheit_or(self, need: bool) -> f32;
}

impl Fahrenheit for f32 {
    fn to_fahrenheit_or(self, need: bool) -> f32 {
        match need {
            true => self * 1.8 + 32.0,
            false => self,
        }
    }
}

#[derive(Deserialize, Debug)]
struct SmartMessages {
    string: Option<String>,
    severity: Option<String>,
}

#[derive(Deserialize, Debug)]
struct SmartCtl {
    messages: Option<Vec<SmartMessages>>,
    exit_status: i32,
}

#[derive(Deserialize, Debug)]
struct TempInfo {
    current: Option<f32>,
}

#[derive(Deserialize, Debug)]
struct PowerOnTime {
    hours: Option<u64>,
}

#[derive(Deserialize, Debug)]
struct SmartDevice {
    #[serde(rename = "type")]
    tp: Option<String>,
}

#[derive(Deserialize, Debug)]
struct UserCapacity {
    bytes: Option<u128>,
}

#[derive(Deserialize, Debug)]
struct SmartStatus {
    passed: Option<bool>,
}

#[derive(Deserialize, Debug)]
struct SmartData {
    #[serde(skip_deserializing)]
    name: String,
    model_name: Option<String>,
    serial_number: Option<String>,
    temperature: Option<TempInfo>,
    power_on_time: Option<PowerOnTime>,
    power_cycle_count: Option<u64>,
    device: Option<SmartDevice>,
    user_capacity: Option<UserCapacity>,
    rotation_rate: Option<u64>,
    firmware_version: Option<String>,
    smart_status: Option<SmartStatus>,
    smartctl: SmartCtl,
}

fn ctable(titles: Option<Vec<&str>>, raw: bool) -> prettytable::Table {
    let mut table = prettytable::Table::new();
    let format = prettytable::format::FormatBuilder::new()
        .column_separator(' ')
        .borders(' ')
        .separators(
            &[prettytable::format::LinePosition::Title],
            prettytable::format::LineSeparator::new('-', '-', '-', '-'),
        )
        .padding(0, 1)
        .build();
    table.set_format(format);
    titles.map(|tt| {
        let mut titlevec: Vec<prettytable::Cell> = Vec::new();
        for t in tt {
            if raw {
                titlevec.push(prettytable::Cell::new(t));
            } else {
                titlevec.push(prettytable::Cell::new(t).style_spec("Fb"));
            }
        }
        table.set_titles(prettytable::Row::new(titlevec));
    });
    table
}

#[cfg(debug_assertions)]
fn smartctl(device: &str) -> Vec<u8> {
    Command::new("sudo")
        .args(&["smartctl", "-a", device, "-j"])
        .output()
        .expect("failed to execute smartctl")
        .stdout
}

#[cfg(not(debug_assertions))]
fn smartctl(device: &str) -> Vec<u8> {
    Command::new("smartctl")
        .args(&["-a", device, "-j"])
        .output()
        .expect("failed to execute smartctl")
        .stdout
}

fn process_temperature(
    t: Option<TempInfo>,
    fahrenheit: bool,
    levels: (f32, f32),
) -> (ColoredString, bool) {
    let mut err = false;
    let ts = t.map_or(empty_c!(), |temp| {
        temp.current.map_or(empty_c!(), |tcur_c| {
            let tcur = tcur_c.to_fahrenheit_or(fahrenheit);
            let s = match fahrenheit {
                true => format!("{:.0} F", tcur),
                false => format!("{:.0} C", tcur),
            };
            if tcur >= levels.1 {
                err = true;
                s.red().bold()
            } else if tcur >= levels.0 {
                s.yellow().bold()
            } else {
                s.green().bold()
            }
        })
    });
    (ts, err)
}

#[derive(Clap)]
#[clap(version = VERSION, about = "https://github.com/alttch/shd")]
struct Opts {
    #[clap(long, about = "Warning temperature, default 40 C (50 for nvme)")]
    temp_warn: Option<f32>,
    #[clap(long, about = "Critical temperature, default 50 C (60 for nvme)")]
    temp_crit: Option<f32>,
    #[clap(short = 'f', long, about = "Use fahrenheit temperatures")]
    fahrenheit: bool,
    #[clap(short = 'R', long, about = "Suppress colors")]
    raw: bool,
    #[clap(short = 'y', long, about = "Display full info")]
    full: bool,
    #[clap(
        short = 'e',
        long,
        about = "Display only disks with errors / critical temperature"
    )]
    errors: bool,
    #[clap(short = 's', long, about = "Suppress header")]
    no_header: bool,
}

fn main() {
    let mut exit_code = EXIT_CODE_NORMAL;
    let opts = Opts::parse();
    if opts.raw {
        SHOULD_COLORIZE.set_override(false);
    }
    let mut devices = Vec::<SmartData>::new();
    for m in vec!["nvme[0-999]", "sd[a-z]", "hd[a-z]"] {
        for entry in glob(&format!("/dev/{}", m)).expect(&format!("Failed to read path {}", m)) {
            match entry {
                Ok(path) => {
                    let p = path.to_str().unwrap();
                    if SHOULD_COLORIZE.should_colorize() {
                        print!(": {}", p.cyan());
                        io::stdout().flush().unwrap();
                    }
                    let data = smartctl(p);
                    let mut smartdata: SmartData = serde_json::from_slice(&data)
                        .map_err(|e| {
                            println!("Unable to get device {} info: {}", p, e);
                        })
                        .unwrap();
                    if SHOULD_COLORIZE.should_colorize() {
                        io::stdout().write(&[0x0d, 0x1b, 0x5b, 0x4b]).unwrap();
                        io::stdout().flush().unwrap();
                    }
                    if smartdata.smartctl.exit_status != 0 {
                        exit_code = EXIT_CODE_SMARTCTL;
                        println!("{}", format!("Unable to read device {} info", p).red());
                        smartdata.smartctl.messages.map(|messages| {
                            for m in messages {
                                m.string.map(|s| println!("{}", s));
                            }
                        });
                    } else {
                        smartdata.name = path.file_name().unwrap().to_str().unwrap().to_owned();
                        devices.push(smartdata);
                    }
                }
                Err(e) => {
                    panic!("{}", e);
                }
            }
        }
    }
    let mut titles = vec!["Disk", "Model", "Serial", "Temp"];
    if opts.full {
        titles.extend(vec!["PoH", "PCC", "Int", "Capacity", "RRate", "Firmware"]);
    }
    if !devices.is_empty() {
        let mut table = ctable(
            match opts.no_header {
                true => None,
                false => Some(titles),
            },
            opts.raw,
        );
        let temp_warn = opts
            .temp_warn
            .unwrap_or(TEMP_WARN_DEFAULT_C.to_fahrenheit_or(opts.fahrenheit));
        let temp_crit = opts
            .temp_warn
            .unwrap_or(TEMP_CRIT_DEFAULT_C.to_fahrenheit_or(opts.fahrenheit));
        let temp_warn_nvme = opts
            .temp_warn
            .unwrap_or(TEMP_WARN_DEFAULT_C_NVME.to_fahrenheit_or(opts.fahrenheit));
        let temp_crit_nvme = opts
            .temp_warn
            .unwrap_or(TEMP_CRIT_DEFAULT_C_NVME.to_fahrenheit_or(opts.fahrenheit));
        for d in devices {
            let device_tp = d.device.map_or(empty!(), |v| v.tp.unwrap_or_default());
            let (temp, temp_err) = process_temperature(
                d.temperature,
                opts.fahrenheit,
                match device_tp.as_str() {
                    "nvme" => (temp_warn_nvme, temp_crit_nvme),
                    _ => (temp_warn, temp_crit),
                },
            );
            if temp_err && exit_code != EXIT_CODE_ERRORS {
                exit_code = EXIT_CODE_TEMP;
            }
            let smart_status = d
                .smart_status
                .map_or(false, |s| s.passed.unwrap_or_default());
            if !smart_status {
                exit_code = EXIT_CODE_ERRORS;
            }
            if !opts.errors || (temp_err || !smart_status) {
                macro_rules! mark_err {
                    ($s: expr, $err: expr) => {
                        match $err {
                            true => $s.red(),
                            false => $s,
                        }
                    };
                }
                let mut cells = vec![
                    cell!(mark_err!(d.name.cyan(), !smart_status)),
                    cell!(mark_err!(
                        d.model_name.unwrap_or_default().white(),
                        !smart_status
                    )),
                    cell!(mark_err!(
                        d.serial_number.unwrap_or_default().cyan().bold(),
                        !smart_status
                    )),
                    cell!(temp),
                ];
                if opts.full {
                    cells.extend(vec![
                        cell!(
                            s!(d.power_on_time.unwrap_or(PowerOnTime { hours: None }).hours)
                                .normal()
                        ),
                        cell!(s!(d.power_cycle_count).cyan()),
                        cell!(device_tp.normal()),
                        cell!(d
                            .user_capacity
                            .map_or(empty!(), |v| v.bytes.map_or(empty!(), |b| {
                                let byte = byte_unit::Byte::from_bytes(b);
                                byte.get_appropriate_unit(false).to_string()
                            }))
                            .bold()),
                        cell!(match d.rotation_rate.unwrap_or_default() {
                            0 => empty!(),
                            v @ _ => format!("{}", v),
                        }
                        .magenta()),
                        cell!(d.firmware_version.unwrap_or_default().normal()),
                    ]);
                }
                table.add_row(prettytable::Row::new(cells));
            }
        }
        if !table.is_empty() {
            table.printstd();
        };
    } else {
        println!("{}", "No devices available".yellow().bold());
    }
    exit(exit_code);
}
