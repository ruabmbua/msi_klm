/****************************************************************************
    Copyright (c) 2015 Roland Ruckerbauer All Rights Reserved.

    This file is part of msi_klm.

    msi_klm is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    msi_klm is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with msi_klm.  If not, see <http://www.gnu.org/licenses/>.
****************************************************************************/

extern crate msi_klm;
extern crate getopts;

use msi_klm::{KeyboardLights, HidApi, Color, Area};
use std::env;
use getopts::{Options, Matches};

fn main() {
    let api = HidApi::new();
    let mut lights = match api {
        Err(e) => {
            println!("An unexpected error at api initialization occured: {}", e);
            return
        },
        Ok(ref a) => match KeyboardLights::from_hid_api(a) {
            Err(e) => {
                println!("An unexpected error at device opening occured: {}", e);
                return
            },
            Ok(l) => l,
        },
    };
    let mut opts = Options::new();
    let args: Vec<String> = env::args().collect();
    let program = &args[0];
    opts.optflag("h", "help", "print this help menu");
    opts.optopt("l", "left", "set left area of keyboard", "COLOR");
    opts.optopt("m", "middle", "set middle area of keyboard", "COLOR");
    opts.optopt("r", "right", "set right area of keyboard", "COLOR");
    opts.optopt("a", "all", "set all areas of keyboard", "COLOR");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            println!("Error parsing command line arguments: {}", e.to_string());
            return;
        }
    };
    if matches.opt_present("h") {
        print_usage(program, opts);
        return;
    }
    set_light(&matches, "a", &lights);
    set_light(&matches, "l", &lights);
    set_light(&matches, "m", &lights);
    set_light(&matches, "r", &lights);
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn hexstr_to_color(hex: &str) -> Result<Color, &'static str> {
    let mut color: Color = Default::default();
    let mut iter = hex.chars();
    color.r += try!(extract_char_value(iter.next())) * 16;
    color.r += try!(extract_char_value(iter.next()));
    color.g += try!(extract_char_value(iter.next())) * 16;
    color.g += try!(extract_char_value(iter.next()));
    color.b += try!(extract_char_value(iter.next())) * 16;
    color.b += try!(extract_char_value(iter.next()));
    if iter.next().is_some() {
        Err("to many characters for hex color")
    }else {
        Ok(color)
    }
}

fn extract_char_value(c: Option<char>) -> Result<u8, &'static str> {
    match c {
        None => Err("to few characters"),
        Some(c) => {
            match c.to_digit(16) {
                None => Err("character can not be in hex number"),
                Some(d) => Ok(d as u8),
            }
        }
    }
}

fn set_light(matches: &Matches, name: &str, lights: &KeyboardLights) {
    let hex = matches.opt_str(name);
    match hex {
        None => (),
        Some(h) => {
            let color = match hexstr_to_color(&h) {
                Err(e) => {
                    println!("Wrong color format for option -{}: {}", name, e);
                    return;
                },
                Ok(c) => c,
            };
            if name == "a" {
                lights.set_all(color);
            }else {
                lights.set_area(opt_to_area(name), color);
            }
            println!("Successfully set option -{} to {:?}", name, color);
        },
    }
}

fn opt_to_area(opt: &str) -> Area {
    match opt {
        "l" => Area::Left,
        "m" => Area::Middle,
        "r" => Area::Right,
        _ => panic!("Unexpected opt"),
    }
}
