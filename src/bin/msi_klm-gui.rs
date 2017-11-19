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

extern crate msi_klm;
#[cfg(feature = "gui")]
extern crate gtk;

#[cfg(not(feature = "gui"))]
fn main() {
    use std::io::Write;
    writeln!(std::io::stderr(), "msi_klm was built without GUI support.").unwrap();
}

#[cfg(feature = "gui")]
fn main() {
    use std::rc::Rc;
    use std::cell::RefCell;
    use msi_klm::{HidApi, KeyboardLights};

    let api = HidApi::new().unwrap_or_else(|e| {
        println!("An unexpected error at api initialization occured: {}", e);
        std::process::exit(-1);
    });

    let lights = KeyboardLights::from_hid_api(&api)
        .unwrap_or_else(|e| {
            println!("An unexpected error at device opening occured: {}", e);
            std::process::exit(-1);
        });

    gui::launch(&lights);
}

#[cfg(feature = "gui")]
mod gui {
    use gtk;
    use gtk::prelude::*;
    use msi_klm::{KeyboardLights, Area, Mode, Color, HidApi};
    use std::rc::Rc;
    use std::cell::RefCell;

    macro_rules! glade_import {
        ($name:ident, $gtype:path, $builder:ident) => {
            let $name: $gtype = $builder.get_object(stringify!($name)).unwrap();
        };
    }

    macro_rules! clone {
        (@param _) => ( _ );
        (@param $x:ident) => ( $x );
        ($($n:ident),+ => move || $body:expr) => (
            {
                $( let $n = $n.clone(); )+
                move || $body
            }
        );
        ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
            {
                $( let $n = $n.clone(); )+
                move |$(clone!(@param $p),)+| $body
            }
        );
    }

    pub fn launch(device: &KeyboardLights) {
        gtk::init().unwrap();
        let glade_src = include_str!("../../res/main.glade");

        let builder = gtk::Builder::new_from_string(glade_src);

        glade_import!(main_window, gtk::ApplicationWindow, builder);

        glade_import!(btn_toggle, gtk::Button, builder);

        main_window.connect_delete_event(|_, _| {
            gtk::main_quit();
            gtk::Inhibit(false)
        });

        btn_toggle.connect_clicked(clone!(device => move |_| {
            println!("Toggled");
            device.set_mode(Mode::Off);
        }));

        main_window.show_all();

        gtk::main();
    }
}