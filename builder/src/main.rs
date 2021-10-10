use shared_library_builder::{Library, LibraryCompilationContext, LibraryGitLocation, LibraryLocation, LibraryTarget, PathLocation, RustLibrary};
use std::error::Error;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    let library = RustLibrary::new(
        "Process",
        LibraryLocation::Path(PathLocation::new(std::env::current_dir().unwrap())),
    );

    let context = LibraryCompilationContext::new("target", "target", LibraryTarget::for_current_platform(), false);
    let compiled_library = library.compile(&context)?;
    println!("Compiled {}", compiled_library.display());
    Ok(())
}
