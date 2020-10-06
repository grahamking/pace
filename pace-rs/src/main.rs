/*
 * Copyright 2020 Graham King
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 * For full license details see <http://www.gnu.org/licenses/>.
 */

use std::collections::HashMap;
use std::env;
use std::fmt;

fn usage() {
    print!("pace has two modes: pace and distance.\n");
    print!("DISTANCE MODE: `pace 10k 1h`\n");
    print!("Usage: pace [distance] [time]\n");
    print!("distance:\n");
    print!("\tnumber followed by 'k' for kilometers, e.g. 10k\n");
    print!("\tnumber followed by 'm' for miles, e.g. 26.2m\n");
    print!("\tspecial word 'marathon' or 'half'\n");
    print!("time:\n");
    print!("\tnumber followed by 'h' for hours\n");
    print!("\tnumber followed by 'm' for minutes\n");
    print!("\thh:mm format, e.g. 3:30\n");
    print!("PACE MODE: `pace 4:30k`\n");
    print!("Usage: pace [pace]\n");
    print!("pace:\n");
    print!("\tmin:secs followed by 'k' for per kilometer, e.g. 5:30k\n");
    print!("\tmins:secs followed by 'm' for per mile, e.g. 7:00m\n");
}

// d_raw_param: The distance, one of:
//  - "marathon" or "half"
//  - "<number>k", e.g. "5k"
//  - "<number>m", e.g "26.2m"
//  t_raw_param: The time, in format 1h05m10s, all parts optional.
fn do_distance(d_raw_param: &str, t_raw_param: &str) {
    let d_raw = if d_raw_param == "marathon" {
        "42.2k"
    } else if d_raw_param == "half" {
        "21.1k"
    } else {
        d_raw_param
    };
    let c = d_raw.chars().last().unwrap();
    let du = match c {
        'm' => DistUnit::Miles,
        'k' => DistUnit::Kilometers,
        x => {
            println!("Invalid distance unit '{}'. Must be 'k' or 'm'", x);
            return;
        }
    };
    let dist_k: f64;
    let dist_m: f64;
    match du {
        DistUnit::Miles => {
            dist_m = d_raw[0..d_raw.len() - 1].parse().unwrap();
            dist_k = dist_m * 1.609;
        }
        DistUnit::Kilometers => {
            dist_k = d_raw[0..d_raw.len() - 1].parse().unwrap();
            dist_m = dist_k * 0.62137119;
        }
    }

    let sec = f64::from(t_to_sec(t_raw_param));
    let sec_k = sec / dist_k;
    let sec_m = sec / dist_m;

    println!("{:.2} km / {:.2} miles in {}:", dist_k, dist_m, t_raw_param);
    println!("\t{} / km", sec_to_string(sec_k));
    println!("\t{} / mile", sec_to_string(sec_m));
}

#[derive(PartialEq)]
enum DistUnit {
    Miles,
    Kilometers,
}
impl fmt::Display for DistUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DistUnit::Miles => write!(f, "miles"),
            DistUnit::Kilometers => write!(f, "km"),
        }
    }
}

// Seconds to human readable string
fn sec_to_string(sec: f64) -> String {
    let mut out = String::new();
    let mut rest = sec;
    let mut h = 0.0;
    // are there any hours?
    if rest >= 3600.0 {
        h = (rest / 3600.0).floor();
        out += &format!("{}h", h);
        rest -= h * 3600.0;
    }
    let m = (rest / 60.0).floor();
    // only add minutes if there are hours or minutes
    if h >= 1.0 || m >= 1.0 {
        if h >= 1.0 && m < 10.0 {
            out += &"0";
        }
        out += &format!("{}m", m);
        rest -= m * 60.0;
    }
    if rest >= 1.0 {
        out += &format!("{:02}s", rest.floor());
    }
    out
}

#[derive(PartialEq, Eq, Hash)]
enum Distance {
    FiftyK,
    Marathon,
    HalfMarathon,
    TenK,
    FiveK,
}
impl Distance {
    fn all() -> Vec<Distance> {
        return vec![
            Distance::FiftyK,
            Distance::Marathon,
            Distance::HalfMarathon,
            Distance::TenK,
            Distance::FiveK,
        ];
    }
    fn name(&self) -> &str {
        match self {
            Distance::FiftyK => "50k",
            Distance::Marathon => "Marathon",
            Distance::HalfMarathon => "Half-Marathon",
            Distance::TenK => "10k",
            Distance::FiveK => "5k",
        }
    }
    fn km(&self) -> f64 {
        match self {
            Distance::FiftyK => 50.0,
            Distance::Marathon => 42.2,
            Distance::HalfMarathon => 21.1,
            Distance::TenK => 10.0,
            Distance::FiveK => 5.0,
        }
    }
}

// pace is in seconds per km
fn get_distances(pace: f64) -> HashMap<Distance, f64> {
    let mut m = HashMap::new();
    for di in Distance::all() {
        let time = di.km() * pace;
        m.insert(di, time);
    }
    m
}

// pace is in seconds per km
fn display_distances(pace: f64) {
    let mut m = get_distances(pace);
    println!("At that pace:");
    for di in Distance::all() {
        let time_s = m.remove(&di).unwrap();
        println!("\t{:15}{}", di.name(), sec_to_string(time_s));
    }
}

fn convert_to_per_km(per_mile: f64) -> f64 {
    per_mile * 0.62137119223733
}

fn t_to_sec(t: &str) -> u32 {
    let mut rest = String::from(t);
    let mut secs: u32 = 0;
    if rest.contains(":") {
        let parts: Vec<&str> = rest.split(":").collect();
        if parts.len() != 2 {
            // TODO: return error
            println!("{} invalid format, expected e.g. '7:30'", t);
            return 0;
        }
        let min: u32 = parts[0].parse().unwrap();
        let sec: u32 = parts[1].parse().unwrap();
        return min * 60 + sec;
    }
    if rest.contains("h") {
        let sp: Vec<&str> = rest.split("h").collect();
        let hours: u32 = sp[0].parse().unwrap();
        secs += hours * 3600;
        rest = String::from(sp[1]);
    }
    if rest.contains("m") {
        let sp: Vec<&str> = rest.split("m").collect();
        let mins: u32 = sp[0].parse().unwrap();
        secs += mins * 60;
        rest = String::from(sp[1]);
    }
    if rest.contains("s") {
        // final s is optional
        let sp: Vec<&str> = rest.split("s").collect();
        rest = String::from(sp[0]);
    }
    if rest.len() > 0 {
        secs += rest.parse::<u32>().unwrap();
    }
    secs
}

// p is running page, format min:sec[m|k].
fn do_pace(p: &str) {
    let c = p.chars().last().unwrap();
    let du = match c {
        'm' => DistUnit::Miles,
        'k' => DistUnit::Kilometers,
        _ => {
            println!("Invalid pace unit '{}'. Must be 'k' or 'm'", c);
            return;
        }
    };
    let mut sec = f64::from(t_to_sec(&p[0..p.len() - 1]));
    if du == DistUnit::Miles {
        sec = convert_to_per_km(sec); // because all the distances are in km
    }
    display_distances(sec);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => do_pace(&args[1]),
        3 => do_distance(&args[1], &args[2]),
        _ => usage(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sec_to_string() {
        // just hours
        assert_eq!("1h00m", sec_to_string(3600.0));
        // just minutes
        assert_eq!("1m", sec_to_string(60.0));
        // just seconds
        assert_eq!("01s", sec_to_string(1.0));
        // normal
        assert_eq!("3h05m12s", sec_to_string(3.0 * 3600.0 + 5.0 * 60.0 + 12.0));
        // no minutes
        assert_eq!("3h00m01s", sec_to_string(3.0 * 3600.0 + 1.0));
        // fractional seconds
        assert_eq!("3h00m03s", sec_to_string(10803.409));
    }

    #[test]
    fn test_get_distances() {
        let three_hour_marathon = convert_to_per_km(6.0 * 60.0 + 52.0);
        let d = get_distances(three_hour_marathon);
        let m_time = *d.get(&Distance::Marathon).unwrap();
        assert_eq!(m_time.floor(), 3.0 * 3600.0 + 3.0);
    }

    #[test]
    fn test_parse_time() {
        // seconds
        assert_eq!(t_to_sec("1s"), 1);
        // minutes
        assert_eq!(t_to_sec("10m"), 600);
        // hours
        assert_eq!(t_to_sec("1h"), 3600);
        // all
        assert_eq!(t_to_sec("1h01m01"), 3600 + 60 + 1);
        // with colon for minutes
        assert_eq!(t_to_sec("1:01"), 61);
    }
}
