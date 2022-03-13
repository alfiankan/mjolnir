pub mod mjl;
mod btree;

use crate::mjl::types;

use crate::mjl::cli;
use crate::mjl::engine::Engine;

fn main() {
    // mql => mjolnir query language
    cli::start_cli();
}
