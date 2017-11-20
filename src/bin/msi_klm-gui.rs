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
#[cfg(feature = "gui")]
extern crate gdk;

#[cfg(not(feature = "gui"))]
fn main() {
    use std::io::Write;
    writeln!(std::io::stderr(), "msi_klm was built without GUI support.").unwrap();
}

#[cfg(feature = "gui")]
fn main() {
    use msi_klm::{HidApi, KeyboardLights, State};
    use std::rc::Rc;
    use std::cell::RefCell;

    let state = Rc::new(RefCell::new(State::load_from_config()));

    let api = HidApi::new();
    let lights = match api {
        Err(e) => {
            println!("An unexpected error at api initialization occured: {}", e);
            ::std::process::exit(-1);
        }
        Ok(ref a) => {
            match KeyboardLights::from_hid_api(a) {
                Err(e) => {
                    println!("An unexpected error at device opening occured: {}", e);
                    ::std::process::exit(-1);
                }
                Ok(l) => l,
            }
        }
    };

    gui::launch(lights, state.clone());


    let state_brw = state.borrow();
    if let Err(e) = state_brw.store_into_config() {
        println!("An error happend, while storing state to disk: {}", e.description());
    }
}

#[cfg(feature = "gui")]
mod gui {
    use gtk;
    use gdk;
    use gtk::prelude::*;
    use msi_klm::{KeyboardLights, Area, Mode, Color, HidApi, State};
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

    fn into_gdk_color(c: &Color) -> gdk::Color {
        gdk::Color {
            pixel: 0,
            red: ((c.r as u16) << 8) | c.r as u16,
            green: ((c.g as u16) << 8) | c.g as u16,
            blue: ((c.b as u16) << 8) | c.b as u16,
        }
    }

    pub fn launch(lights: KeyboardLights, state: Rc<RefCell<State>>) {
        gtk::init().unwrap();
        let glade_src = include_str!("../../res/main.glade");

        let builder = gtk::Builder::new_from_string(glade_src);

        glade_import!(main_window, gtk::ApplicationWindow, builder);

        glade_import!(switch_power, gtk::Switch, builder);

        glade_import!(color_left, gtk::ColorButton, builder);
        glade_import!(color_center, gtk::ColorButton, builder);
        glade_import!(color_right, gtk::ColorButton, builder);

        glade_import!(scale_brightness, gtk::ScaleButton, builder);

        switch_power.set_state(if state.borrow().mode == Mode::Default { true } else { false });
        scale_brightness.set_value(state.borrow().brightness as f64 * 100.0);
        color_left.set_color(&into_gdk_color(state.borrow().areas.get(&Area::Left).unwrap()));
        color_center.set_color(&into_gdk_color(state.borrow().areas.get(&Area::Middle).unwrap()));
        color_right.set_color(&into_gdk_color(state.borrow().areas.get(&Area::Right).unwrap()));

        main_window.connect_delete_event(|_, _| {
            gtk::main_quit();
            gtk::Inhibit(false)
        });

        let lights = Rc::new(lights);

        main_window.show_all();

        gtk::main();
    }
}