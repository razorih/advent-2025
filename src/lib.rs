use std::{
    io::{self, ErrorKind},
    path::{Path, PathBuf},
};

pub fn read_input_from_env() -> io::Result<String> {
    let filename = get_filename_from_args()?;

    if filename == "-" {
        return std::io::read_to_string(std::io::stdin());
    }

    let resolved = resolve_path(&filename)?;

    std::fs::read_to_string(resolved)
}

fn get_filename_from_args() -> io::Result<String> {
    std::env::args().nth(1).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::Other,
            r#"expected input file path or "-" as first argument"#,
        )
    })
}

fn resolve_path(path: impl AsRef<Path>) -> io::Result<PathBuf> {
    let path = path.as_ref();

    // need to handle three different cases (in priority order):
    //   1. absolute paths
    //   2. relative paths
    //   3. paths relative to {current_dir}/inputs/

    // 1.
    if path.is_absolute() && path.try_exists()? {
        return Ok(path.to_path_buf());
    }

    // 2.
    if path.try_exists()? {
        return Ok(path.to_path_buf());
    }

    // 3.
    let mut base = std::env::current_dir()?;
    base.push("inputs/");
    base.push(path);

    if base.try_exists()? {
        return Ok(base.to_path_buf());
    }

    Err(io::Error::from(ErrorKind::NotFound))
}
