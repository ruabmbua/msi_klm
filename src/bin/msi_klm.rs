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

use msi_klm::{KeyboardLights, HidApi, Color};

fn main() {
    let api = HidApi::new().unwrap();
    let mut lights = KeyboardLights::from_hid_api(&api).unwrap();
    lights.set_all(Color::new(0, 0, 0));
    lights.upload();
}
