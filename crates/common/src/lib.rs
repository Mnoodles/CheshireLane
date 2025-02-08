pub mod logging;
pub mod toml;
pub mod command;
pub mod time;

pub fn banner() {
    println!(r#"
 ____     __                      __
/\  _`\  /\ \                    /\ \      __
\ \ \/\_\\ \ \___      __    ____\ \ \___ /\_\  _ __    __
 \ \ \/_/_\ \  _ `\  /'__`\ /',__\\ \  _ `\/\ \/\`'__\/'__`\
  \ \ \L\ \\ \ \ \ \/\  __//\__, `\\ \ \ \ \ \ \ \ \//\  __/
   \ \____/ \ \_\ \_\ \____\/\____/ \ \_\ \_\ \_\ \_\\ \____\
    \/___/   \/_/\/_/\/____/\/___/   \/_/\/_/\/_/\/_/ \/____/
    "#);
}

pub fn show_info() {
    logging::info!("Author: {}", env!("CARGO_PKG_AUTHORS"));
    logging::info!("Current version: {}", env!("CARGO_PKG_VERSION"));
}
