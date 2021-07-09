pub mod exec {
	use glob::glob;
	use std::error::Error;

	pub fn get_keyboard_layouts() -> Result<Vec<String>, Box<dyn Error>> {
		let mut list = vec![];

		for entry in glob("/usr/share/kbd/keymaps/**/*.map.gz")? {
			list.append(
				&mut vec![
					entry.unwrap()
						.display().to_string()
						.split("/").last()
						.unwrap().to_string()
						.replace(".map.gz", "")
				]);
		}

		Ok(list)
	}

	pub fn set_keyboard_layout(layout: &str) -> Result<(), Box<dyn Error>> {
		let status = std::process::Command::new("loadkeys")
			.arg(layout)
			.status();
		match status {
			Ok(exit_status) => {
				if exit_status.code() == Some(0) {
					Ok(())
				} else {
					Err("loadkeys exited with non-zero exit code (are you root?)".into())
				}
			},
			Err(e) => Err(Box::new(e))
		}
	}

	pub fn get_timezones() -> Result<Vec<String>, Box<dyn Error>> {
		let output = std::process::Command::new("timedatectl")
			.arg("list-timezones")
			.output().unwrap().stdout;

		let raw_list = String::from_utf8(output).unwrap();
		Ok(raw_list.lines().map(String::from).collect())
	}

	pub fn set_timezone(timezone: &str) -> Result<(), Box<dyn Error>> {
		let status = std::process::Command::new("timedatectl")
			.arg("set-timezone")
			.arg(timezone)
			.status();
		match status {
			Ok(exit_status) => {
				if exit_status.code() == Some(0) {
					Ok(())
				} else {
					Err("timedatectl exited with non-zero exit code (are you root?)".into())
				}
			},
			Err(e) => Err(Box::new(e))
		}
	}
}

pub mod keyboard {
	use cursive::{Cursive, CursiveExt};
	use cursive::view::Scrollable;
	use cursive::views::{Dialog, TextView, SelectView, Button, LinearLayout, DummyView as Spacer};

	pub fn setup(screen: &mut Cursive) {
		match super::exec::get_keyboard_layouts() {
			Ok(layouts) => {
				screen.pop_layer();
				screen.add_layer(
					Dialog::around(
						LinearLayout::vertical()
							.child(TextView::new("Choose a keyboard layout"))
							.child(Spacer)
							.child(
								SelectView::<String>::new()
									.with_all_str(layouts)
									.on_submit(|screen: &mut Cursive, layout: &str| {
										match super::exec::set_keyboard_layout(layout) {
											Ok(_) => {
												super::screens::live_env_setup(screen)
											},
											Err(e) => {
												println!("Error: {}", e);
												screen.add_layer(
													Dialog::info(
														format!("Failed to set keyboard layout: {}", e)
													)
												);
											}
										};
									})
									.scrollable()
							)
							.child(Button::new("Back", super::screens::live_env_setup))
					)
				);

				screen.run();
			},
			Err(e) => {
				println!("Error: {}", e);
				screen.add_layer(
					Dialog::info(format!("Failed to list keyboard layout: {}", e))
				);
				screen.run();
				super::screens::live_env_setup(screen);
			}
		}
	}
}

pub mod timezone {
	use cursive::{Cursive, CursiveExt};
	use cursive::view::Scrollable;
	use cursive::views::{Dialog, TextView, SelectView, Button, LinearLayout, DummyView as Spacer};

	pub fn setup(screen: &mut Cursive) {
		match super::exec::get_timezones() {
			Ok(timezones) => {
				screen.pop_layer();
				screen.add_layer(
					Dialog::around(
						LinearLayout::vertical()
							.child(TextView::new("Choose your timezone"))
							.child(Spacer)
							.child(
								SelectView::<String>::new()
									.with_all_str(timezones)
									.on_submit(|screen: &mut Cursive, timezone: &str| {
										match super::exec::set_timezone(timezone) {
											Ok(_) => {
												super::screens::live_env_setup(screen)
											},
											Err(e) => {
												println!("Error: {}", e);
												screen.add_layer(
													Dialog::info(
														format!("Failed to set timezone: {}", e)
													)
												);
											}
										};
									})
									.scrollable()
							)
							.child(Button::new("Back", super::screens::live_env_setup))
					)
				);

				screen.run();
			},
			Err(e) => {
				println!("Error: {}", e);
				screen.add_layer(
					Dialog::info(format!("Failed to list timezones: {}", e))
				);
				screen.run();
				super::screens::live_env_setup(screen);
			}
		}
	}
}

pub mod screens {
	use cursive::Cursive;
	use cursive::views::{Dialog, TextView, Button, LinearLayout, DummyView as Spacer};	
	pub fn intro(screen: &mut Cursive) {
		screen.pop_layer();
		screen.add_layer(
			Dialog::around(
				LinearLayout::vertical()
					.child(TextView::new(
						"Hi there! Welcome to Blop; an easier way to install Arch Linux."
					))
					.child(Spacer)
					.child(Button::new("Begin Installation", self::live_env_setup))
			).title("BLOP - Arch Linux Installer")
		);
	}

	pub fn live_env_setup(screen: &mut Cursive) {
		screen.pop_layer();
		screen.add_layer(
			Dialog::around(
				LinearLayout::vertical()
					.child(Button::new("Keyboard Layout", super::keyboard::setup))
					.child(Button::new("Timezone", super::timezone::setup))
					.child(Button::new("Next", super::screens::disks_setup))
			).title("Live environment Setup")
		)
	}
	
	pub fn disks_setup(screen: &mut Cursive) {
		screen.pop_layer();
		screen.add_layer(
			Dialog::around(
				LinearLayout::vertical()
					.child(TextView::new(
						"Partition disks using one of the following tools:"
					))
					.child(Spacer)
					.child(Button::new("fdisk", super::disks::fdisk))
					.child(Button::new("gdisk", super::disks::gdisk))
					.child(Button::new("parted", super::disks::parted))
			).title("Disk Setup")
		)
	}
}
fn main() {
	let mut screen = cursive::default();

	screen.add_global_callback('q', |s| s.quit());

	self::screens::intro(&mut screen);
	
	screen.run()
}
