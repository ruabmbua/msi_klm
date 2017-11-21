# msi_klm [![Crates.io](https://img.shields.io/crates/v/msi_klm.svg)](https://crates.io/crates/msi_klm) [![Build Status](https://travis-ci.org/ruabmbua/msi_klm.svg)](https://travis-ci.org/ruabmbua/msi_klm) #

This crate provides access to the keyboard lights of MSI notebooks with Steel
Series gaming keyboards. It is cross platform and compatible with windows and
linux. The goal of this project is the creation of a GUI and a command line
application. These should be able to replace the Steel Series Engine 3 on
windows, and also provide control over keyboard lights for linux users.

This crate is a work in progress, because there are missing features in the API.
The API is feature complete, when the functionality is comparable to the Steel
Series Engine 3.

## Screenshot

![GUI screenshot](https://raw.githubusercontent.com/ruabmbua/msi_klm/stable/screenshot.png)

## Installation and usage

There is an early version of the command line utility included in this project.
To run it you have to install it first. The installation process is only
described for linux, because windows needs some work to improve the process.

### Installation on Arch linux

Coming soon!

### Installation on Linux

#### Install and configure USB access to keyboard controller

First install the libusb library with your system package manager:

* Ubuntu: libusb-1.0-0-dev
* Arch Linux: libusb

Then you have to give all users on the system access to the MSI-EPF USB node.
That is done by copying the *etc/90-msi-epf.rules* file, which is included in
the project onto your system. The path where it should go on your system is
*/etc/udev/rules.d/90-msi-epf.rules*. After copying is finished, you have to
tell udev to reload its rules with the command *udevadm control --reload-rules*,
or just restart your computer.

#### Install GTK3 development libraries (optional)

This is only required, when the GUI of the tool is wanted too.
Please look at your distro documentation, for how to install GTK3 development libraries.

#### Install required compilers and tools

The next step is the installation of the rust compiler suite, and its included
build tool. To do that head over to
[https://www.rust-lang.org/install.html](https://www.rust-lang.org/install.html)
and follow the instructions. I recommend the current stable version.

Also the GNU C compiler is required on the system, to build the application.
Install it with your systems package manager. The package is probably called
*gcc*.

Last software, which is required is git. Also install it with your systems
package manager.

#### Download and compilation

Clone the repository onto your local machine, by
typing in the command *git clone https://github.com/ruabmbua/msi_klm.git*.
Change into the new created folder and run *cargo build --no-default-features --release*.

When you want the GUI of the tool, then do not pass the *--no-default-features* flag.

This should build the library and command line utility with all its dependencies, and leave
a statically linked binary at *target/release/msi_klm*, and when built with UI support,
then also *target/release/msi_klm-gui*. Copy these to
*/usr/local/bin/msi_klm*, and the installation is finished. For the last copy
operation super user rights are required.

### Usage

After the command line utility is installed on your system, calling
*msi_klm --help* in your command line should help you with using the program.

Just an example on how to set the right part of your keyboard to fully red:
*msi_klm --right ff0000 --mode ON*

When compiled with GUI support, then launch *msi_klm-gui*, which should show you the user interface.