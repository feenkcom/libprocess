use std::error::Error;

use shared_library_builder::build_standalone;

use libprocess_builder::latest_libprocess;

fn main() -> Result<(), Box<dyn Error>> {
    build_standalone(|_| Ok(Box::new(latest_libprocess())))
}
