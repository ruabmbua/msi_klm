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
        match config.mode {
            Mode::Default => self.handle_colors(config.colors),
            _ => panic!(""),
        }
        self.set_mode(config.mode);
    }

    fn set_mode(&self, mode: Mode) {
        self.device.send_feature_report(&[1, 2, 65, mode as u8, 0, 0, 0, 0]);
    }

    fn handle_colors(&self, colors: [Option<Color>; 3]) {
        for i in 1..4 {

        }
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
    timer: Option<u8>,
}

impl Configuration {
    pub fn new() -> Self {
        Configuration {
            mode: Mode::Off,
            colors: [None; 3],
            extra_colors: [None; 3],
            timer: None,
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

    pub fn set_timer(&mut self, timer: u8) {self.timer = Some(timer);}
}
