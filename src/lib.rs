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

extern crate hidapi_rust;

pub use hidapi_rust::HidApi;
use hidapi_rust::{HidDevice, c_ushort};

const VENDOR_ID: c_ushort = 6000;
const PRODUCT_ID: c_ushort = 65280;
const SET_TYPE_MODE: u8 = 65;
const SET_TYPE_COLOR: u8 = 64;
const SET_TYPE_ADVANCED: u8 = 68;

pub struct Lights<'a> {
    device: HidDevice<'a>,
}

impl <'a> Lights<'a> {
    pub fn from_hid_api(api: &'a HidApi) -> Result<Lights<'a>, &'static str> {
        let device = try!(api.open(VENDOR_ID, PRODUCT_ID));
        Ok(Lights {
            device: device,
        })
    }

    pub fn apply_config(&self, config: &Configuration) {
        self.set_mode(Mode::Reset);
        for i in 0..3 {
            match config.mode {
                Mode::Default => self.handle_colors(i, config.colors, 1, 0),
                Mode::Game => self.handle_colors(i, [config.colors[0], None, None], 1, 0),
                Mode::Breath => {
                    self.handle_colors(i, config.colors, 2, 0);
                    self.handle_timer(i, config.timer, 2);
                },
                Mode::Wave => {
                    self.handle_colors(i, config.colors, 3, 0);
                    self.handle_timer(i, config.timer, 3);
                    self.handle_colors(i, config.extra_colors, 3, 2)
                }
                Mode::Fade => {
                    self.handle_colors(i, config.colors, 3, 0);
                    self.handle_timer(i, config.timer, 3);
                    self.handle_colors(i, config.extra_colors, 3, 2);
                },
                _ => (),
            }
        }
        self.set_mode(config.mode);
        /*self.set_mode(Mode::Reset);

        self.device.send_feature_report(&[1, 2, 64, 1, 0, 255, 0, 0]);
        //self.device.send_feature_report(&[1, 2, 68, 2, 2, 2, 2, 0]);

        self.device.send_feature_report(&[1, 2, 64, 2, 0, 255, 0, 0]);
        //self.device.send_feature_report(&[1, 2, 68, 4, 2, 2, 2, 0]);

        self.device.send_feature_report(&[1, 2, 64, 3, 0, 255, 0, 0]);
        //self.device.send_feature_report(&[1, 2, 68, 6, 2, 2, 2, 0]);

        self.set_mode(Mode::Breath);*/
    }

    fn set_mode(&self, mode: Mode) {
        self.device.send_feature_report(&[1, 2, SET_TYPE_MODE, mode as u8, 0, 0, 0, 0]);
        println!("{:?}", &[1, 2, SET_TYPE_MODE, mode as u8, 0, 0, 0, 0]);
    }

    fn handle_colors(&self, i: usize, colors: [Option<Color>; 3], size: usize, offset: usize) {
        let mut set_type = SET_TYPE_COLOR;
        if size != 1 {
            set_type = SET_TYPE_ADVANCED;
        }
        match colors[i] {
            Some(color) => {
                self.device.send_feature_report(&[1, 2, set_type,
                        (i * size + offset + 1) as u8, color.r, color.g, color.b, 0]);
                println!("{:?}", &[1, 2, set_type,
                        (i * size + offset + 1) as u8, color.r, color.g, color.b, 0]);
            },
            None => {},
        }
    }

    fn handle_timer(&self, i: usize, timer: u8, size: usize) {
        self.device.send_feature_report(&[1, 2, SET_TYPE_ADVANCED, (i * size + 2) as u8,
                timer, timer, timer, 0]);
                println!("{:?}", &[1, 2, SET_TYPE_ADVANCED, (i * size + 2) as u8,
                        timer, timer, timer, 0]);
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color {
            r: r,
            g: g,
            b: b,
        }
    }
}

#[derive(Copy, Clone)]
pub enum Mode {
    Off = 0,
    Default = 1,
    Game = 2,
    Breath = 3,
    //DontKnowWhat = 4
    Wave = 5,
    Fade = 6,
    Reset = 7,
}

pub enum Area {
    Left = 0,
    Middle = 1,
    Right = 2,
}

pub struct Configuration {
    mode: Mode,
    colors: [Option<Color>; 3],
    extra_colors: [Option<Color>; 3],
    timer: u8,
}

impl Configuration {
    pub fn new() -> Self {
        Configuration {
            mode: Mode::Off,
            colors: [None; 3],
            extra_colors: [None; 3],
            timer: 2,
        }
    }

    pub fn set_mode(&mut self, mode: Mode) {self.mode = mode;}

    pub fn set_color(&mut self, colors: [Option<Color>; 3]) {self.colors = colors;}

    pub fn set_color_all(&mut self, color: Color) {
        self.colors = [Some(color), Some(color), Some(color)];
    }

    pub fn set_color_area(&mut self, color: Color, area: Area) {
        self.colors[area as usize] = Some(color);
    }

    pub fn set_extra_color(&mut self, colors: [Option<Color>; 3]) {self.extra_colors = colors;}

    pub fn set_extra_color_all(&mut self, color: Color) {
        self.extra_colors = [Some(color), Some(color), Some(color)];
    }

    pub fn set_extra_color_area(&mut self, color: Color, area: Area) {
        self.extra_colors[area as usize] = Some(color);
    }

    pub fn set_timer(&mut self, timer: u8) {self.timer = timer;}
}
