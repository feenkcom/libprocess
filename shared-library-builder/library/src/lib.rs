use shared_library_builder::{GitLocation, LibraryLocation, RustLibrary};

pub fn libprocess(version: impl Into<String>) -> RustLibrary {
    RustLibrary::new(
        "Process",
        LibraryLocation::Git(GitLocation::github("feenkcom", "libprocess").tag(version)),
    )
        .package("libprocess")
}
