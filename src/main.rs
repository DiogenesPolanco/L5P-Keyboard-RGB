#![windows_subsystem = "windows"]
use clap::crate_authors;
use clap::crate_version;
use fltk::app;
use std::env;
use std::str::FromStr;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
mod gui;
use gui::keyboard_manager;
mod keyboard_utils;
use clap::{App, Arg, SubCommand};
use gui::enums::Message;
fn main() {
	let matches = App::new("Legion Keyboard Control")
		.version(&crate_version!()[..])
		.author(&crate_authors!()[..])
		.about("Placeholder")
		.subcommand(
			SubCommand::with_name("brightness")
				.about("Changes the brightness")
				.arg(Arg::with_name("value").required(true).help("The value to use. [1-2]").index(1)),
		)
		.subcommand(
			SubCommand::with_name("speed")
				.about("Changes the speed")
				.arg(Arg::with_name("value").required(true).help("The value to use. [1-4]").index(1)),
		)
		.subcommand(
			SubCommand::with_name("effect").about("Changes the effect").arg(
				Arg::with_name("effect")
					.required(true)
					.help(format!("The effect to set. Available options are: {}", crate::gui::builder::EFFECT_LIST.join(", ")).as_str())
					.index(1),
			),
		)
		.get_matches();
	match matches.subcommand_name() {
		Some(sub @ "brightness") => {
			let matches = matches.subcommand_matches(sub).unwrap();
			let brightness = matches.value_of("value").unwrap();
			println!("Brightness {}", brightness);
		}
		Some(sub @ "speed") => {
			let matches = matches.subcommand_matches(sub).unwrap();
			let speed = matches.value_of("value").unwrap();

			println!("Speed {}", speed);
		}
		Some(sub @ "effect") => {
			let matches = matches.subcommand_matches(sub).unwrap();
			let effect = matches.value_of("effect").unwrap();
			println!("Effect {}", effect);
			println!("{}", gui::enums::Effects::from_str(effect).unwrap_or(crate::gui::enums::Effects::Static));
		}
		Some(_) => {
			println!("Other");
		}
		None => {
			let exec_name = env::current_exe().unwrap().file_name().unwrap().to_string_lossy().into_owned();
			println!("No subcommands found, starting in GUI mode. To view the possible subcommands type {} --help", exec_name);
			start_with_gui();
		}
	}
}

fn start_with_gui() {
	let app = app::App::default();

	let (tx, rx) = app::channel::<Message>();
	let stop_signal = Arc::new(AtomicBool::new(false));
	let keyboard = match keyboard_utils::get_keyboard(stop_signal.clone()) {
		Ok(keyboard) => keyboard,
		Err(err) => panic!("{}", err),
	};
	let manager = keyboard_manager::KeyboardManager { keyboard, rx };

	//Windows tray logic
	#[cfg(target_os = "windows")]
	{
		use fltk::prelude::*;
		use tray_item::{IconSource, TrayItem};

		type HWND = *mut std::os::raw::c_void;

		static mut WINDOW: HWND = std::ptr::null_mut();

		let mut win = gui::builder::start_ui(manager, tx, stop_signal);

		unsafe {
			WINDOW = win.raw_handle();
		}
		win.set_callback(|_| {
			extern "C" {
				pub fn ShowWindow(hwnd: HWND, nCmdShow: i32) -> bool;
			}
			unsafe {
				ShowWindow(WINDOW, 0);
			}
		});
		//Create tray icon
		let mut tray = TrayItem::new("Keyboard RGB", IconSource::Resource("trayIcon")).unwrap();

		tray.add_menu_item("Show", move || {
			extern "C" {
				pub fn ShowWindow(hwnd: HWND, nCmdShow: i32) -> bool;
			}
			unsafe {
				ShowWindow(WINDOW, 9);
			}
		})
		.unwrap();

		tray.add_menu_item("Quit", || {
			println!("Quit");
			std::process::exit(0);
		})
		.unwrap();

		//Tray loop
		loop {
			if win.shown() {
				app.run().unwrap();
			} else {
				app::sleep(0.05);
			}
		}
	}

	#[cfg(not(target_os = "windows"))]
	{
		gui::builder::start_ui(manager, tx, stop_signal);
		app.run().unwrap();
	}
}
