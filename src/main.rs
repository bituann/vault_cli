use crate::cli::cli as cli_app;

mod cli;
mod services;
mod utils;

fn main() {
	cli_app::run();
}
