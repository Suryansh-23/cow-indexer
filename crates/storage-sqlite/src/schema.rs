use include_dir::{Dir, include_dir};
use std::{path::Path, sync::LazyLock};

MIGRATIONS_DIR: &'static str = Path::new("$CARGO_MANIFEST_DIR")
    .join("../../migrations")
    .to_str()
    .unwrap();

// Define migrations. These are applied atomically.
static MIGRATIONS: LazyLock<Migrations<'static>> =
    LazyLock::new(|| Migrations::from_directory(&MIGRATIONS_DIR).unwrap());
