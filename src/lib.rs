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
use hidapi_rust::HidDevice;
use std::ops::Deref;
use OwnedOrBorrowed::{Owned, Borrowed};

enum OwnedOrBorrowed<'a, T: 'a> {
    Owned(T),
    Borrowed(&'a T),
}

impl <'a, T> Deref for OwnedOrBorrowed<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        match self {
            &Owned(ref o) => o,
            &Borrowed(b) => b,
        }
    }
}

#[derive(Default)]
pub struct Color(u8, u8, u8);

pub struct KeyboardLights<'a> {
    left: Color,
    middle: Color,
    right: Color,
    api: OwnedOrBorrowed<'a, HidApi>,
    device: HidDevice,
}

impl <'a> KeyboardLights<'a> {
    pub fn new() -> Result<KeyboardLights<'a>, &'static str> {
        let api = try!(HidApi::new());
        let device = try!(api.open(6000, 65280));
        Ok(KeyboardLights {
            left: Default::default(),
            middle: Default::default(),
            right: Default::default(),
            api: Owned(api),
            device: device,
        })
    }
}
