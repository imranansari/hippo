extern crate "rustc-serialize" as rustc_serialize;
extern crate docopt;
extern crate rusqlite;
extern crate time;

use docopt::Docopt;
pub use time::Timespec;
pub use conductor::{Item, ItemId};

use conductor::Conductor;

mod conductor;
mod persist;

static USAGE: &'static str = "
Usage:
  hippo [options]
  hippo review [<N>]
  hippo add <description>
  hippo edit <id> <description>
  hippo remove <id>
  hippo list

Options:
  -h, --help    Show this screen.
";

#[derive(RustcDecodable, Debug)]
struct Args {
    arg_description: Option<String>,
    arg_id: Option<String>,
    arg_N: Option<String>,
    cmd_add: bool,
    cmd_edit: bool,
    cmd_remove: bool,
    cmd_list: bool,
    cmd_review: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());

    let default_review_num = 20;
    println!("{:?}", args);
}
