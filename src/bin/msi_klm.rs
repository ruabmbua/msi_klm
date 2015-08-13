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
}
