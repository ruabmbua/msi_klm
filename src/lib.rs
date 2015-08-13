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

pub enum Area {
    Left,
    Middle,
    Right,
}

impl Area {
    pub fn to_number(self) -> u8 {
        match self {
            Area::Left => 1,
            Area::Middle => 2,
            Area::Right => 3,
        }
    }
}

#[derive(Default, Copy, Clone)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
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

pub struct KeyboardLights<'a> {
    device: HidDevice<'a>,
}

impl <'a> KeyboardLights<'a> {
    pub fn from_hid_api(api: &'a HidApi) -> Result<KeyboardLights<'a>, &'static str> {
        let device = try!(api.open(VENDOR_ID, PRODUCT_ID));
        Ok(KeyboardLights {
            device: device,
        })
    }

    pub fn set_area(&mut self, area: Area, color: Color) {
        self.device.send_feature_report(&[1, 2, 65, 7, 0, 0, 0, 0]);
        self.device.send_feature_report(&[1, 2, 64, area.to_number(), color.r, color.g, color.b,
                0]);
        self.device.send_feature_report(&[1, 2, 65, 1, 0, 0, 0, 0]);
    }

    pub fn set_all(&mut self, color: Color) {
        self.device.send_feature_report(&[1, 2, 65, 7, 0, 0, 0, 0]);
        for i in 1..4 {
            self.device.send_feature_report(&[1, 2, 64, i, color.r, color.g, color.b, 0]);
        }
        self.device.send_feature_report(&[1, 2, 65, 1, 0, 0, 0, 0]);
    }
}
