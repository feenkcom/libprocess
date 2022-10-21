use shared_library_builder::{GitLocation, LibraryLocation, RustLibrary};

pub fn libprocess(version: Option<impl Into<String>>) -> RustLibrary {
    RustLibrary::new(
        "Process",
        LibraryLocation::Git(GitLocation::github("feenkcom", "libprocess").tag_or_latest(version)),
    )
    .package("libprocess")
}

pub fn latest_libprocess() -> RustLibrary {
    let version: Option<String> = None;
    libprocess(version)
}
