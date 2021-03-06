/// **************************************************************************
/// Copyright (c) 2017 Roland Ruckerbauer All Rights Reserved.
///
/// This file is part of msi_klm.
///
/// msi_klm is free software: you can redistribute it and/or modify
/// it under the terms of the GNU General Public License as published by
/// the Free Software Foundation, either version 3 of the License, or
/// (at your option) any later version.
///
/// msi_klm is distributed in the hope that it will be useful,
/// but WITHOUT ANY WARRANTY; without even the implied warranty of
/// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
/// GNU General Public License for more details.
///
/// You should have received a copy of the GNU General Public License
/// along with msi_klm.  If not, see <http://www.gnu.org/licenses/>.
/// *************************************************************************

extern crate hidapi as hidapi_rust;
extern crate libc;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate app_dirs;

pub use hidapi_rust::HidApi;
use hidapi_rust::HidDevice;
use libc::c_ushort;
use std::cell::RefCell;
use std::collections::HashMap;
use app_dirs::{AppDataType, AppInfo};
use std::fs::File;

const VENDOR_ID: c_ushort = 6000;
const PRODUCT_ID: c_ushort = 65280;

#[derive(Serialize, Deserialize, Clone)]
pub struct State {
    pub areas: HashMap<Area, Color>,
    pub brightness: f32,
    pub mode: Mode,
}

impl State {
    fn load_from_config() -> State {
        if let Ok(mut p) = app_dirs::app_dir(AppDataType::UserCache, &AppInfo {
            name: "msi_klm",
            author: "Roland Ruckerbauer",
        }, "/") {
            p.push("state.json");
            match File::open(p.as_path()) {
                Ok(f) => {
                    serde_json::from_reader(f)
                        .unwrap_or(State::default())
                }
                Err(_) => State::default(),
            }
        } else {
            State::default()
        }
    }

    fn store_into_config(&self) -> Result<(), Box<std::error::Error>> {
        let mut p = app_dirs::app_dir(AppDataType::UserCache, &AppInfo {
            name: "msi_klm",
            author: "Roland Ruckerbauer",
        }, "/")?;
        p.push("state.json");

        let f = File::create(p.as_path())?;
        serde_json::to_writer(f, self)?;
        Ok(())
    }
}

impl Default for State {
    fn default() -> Self {
        let mut hm = HashMap::new();
        let color = Color::new(0xff, 0, 0);
        hm.insert(Area::Left, color);
        hm.insert(Area::Middle, color);
        hm.insert(Area::Right, color);

        State {
            areas: hm,
            brightness: 1.0,
            mode: Mode::Default,
        }
    }
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Area {
    Left,
    Middle,
    Right,
}

#[derive(PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum Mode {
    Reset = 7,
    Off = 0,
    Default = 1,
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

#[derive(Default, Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r: r, g: g, b: b }
    }

    pub fn apply_brightness(&self, brightness: f32) -> Color {
        Color {
            r: (self.r as f32 * brightness) as u8,
            g: (self.g as f32 * brightness) as u8,
            b: (self.b as f32 * brightness) as u8,
        }
    }
}

pub struct KeyboardLights {
    device: HidDevice,
    pub state: RefCell<State>,
}

impl KeyboardLights {
    pub fn from_hid_api(api: &HidApi) -> Result<KeyboardLights, &'static str> {
        let device = api.open(VENDOR_ID, PRODUCT_ID)?;
        Ok(KeyboardLights { device: device, state: RefCell::new(State::load_from_config()) })
    }

    pub fn set_mode(&self, mode: Mode) {
        self.device.send_feature_report(&[1, 2, 65, mode as u8, 0, 0, 0, 0]).unwrap();
        self.state.borrow_mut().mode = mode;
    }

    pub fn set_area(&self, area: Area, color: Color) {
        let real = color.apply_brightness(self.state.borrow().brightness);
        self.device
            .send_feature_report(&[1, 2, 64, area.to_number(), real.r, real.g, real.b, 0]).unwrap();
        *self.state.borrow_mut().areas.get_mut(&area).unwrap() = color;
    }

    pub fn set_all(&self, color: Color) {
        let real = color.apply_brightness(self.state.borrow().brightness);

        let mut brw = self.state.borrow_mut();

        for area in [Area::Left, Area::Middle, Area::Right].iter() {
            self.device.send_feature_report(&[1, 2, 64, area.to_number(),
                real.r, real.g, real.b, 0]).unwrap();

            *brw.areas.get_mut(area).unwrap() = color;
        }
    }

    pub fn set_brightness(&self, brightness: f32) {
        self.state.borrow_mut().brightness = brightness;

        for area in [Area::Left, Area::Middle, Area::Right].iter() {
            let real = self.state.borrow().areas.get(area)
                .unwrap().apply_brightness(brightness);

            self.device.send_feature_report(&[1, 2, 64, area.to_number(),
                real.r, real.g, real.b, 0]).unwrap();
        }
    }

    pub fn restore_state(&self) {
        let state2;
        {
            state2 = self.state.borrow().clone();
        }

        self.set_mode(state2.mode);
        self.set_brightness(state2.brightness);

        for (k, v) in state2.areas.iter() {
            self.set_area(*k, *v);
        }
    }

    pub fn disk_commit_state(&self) {
        self.state.borrow().store_into_config().unwrap();
    }
}
