use clap::Parser;
use std::fs::{self, File};
use std::io;
use std::path::Path;
use std::process::{Command, Stdio};

/// Take a screenshot.
#[derive(Clone, Debug, Parser)]
pub enum Args {
    /// Select the region.
    Selection,
    /// The entire screen.
    Everything,
}

struct Guard<'a>(&'a Path);

impl Drop for Guard<'_> {
    fn drop(&mut self) {
        fs::remove_file(self.0).ok();
    }
}

fn main() {
    let args = Args::parse();

    let path = tempfile::env::temp_dir().join(".screenshot");
    let _guard = match File::create_new(&path) {
        Ok(_file) => Guard(&path),
        Err(_error) => {
            eprintln!("screenshot: in use");

            return;
        }
    };

    let region = match args {
        Args::Selection => match select_region() {
            Ok(region) => Some(region),
            Err(error) => {
                eprintln!("slurp: {error}");

                return;
            }
        },
        Args::Everything => None,
    };

    if let Err(error) = screenshot_and_copy(region) {
        eprintln!("grim & wl-copy: {error}");
    }
}

fn select_region() -> io::Result<String> {
    let output = Command::new("slurp")
        // display `width x height`
        .arg("-d")
        // non-selection colour
        .args(["-b", "000000b0"])
        // selection border, and text colour
        .args(["-c", "ff0000"])
        // font
        .args(["-F", "Product Sans"])
        // selection border width
        .args(["-w", "1"])
        .output()?;

    Ok(String::from_utf8_lossy(&output.stdout).trim().into())
}

fn screenshot_and_copy(region: Option<String>) -> io::Result<()> {
    let mut command = Command::new("grim");

    if let Some(region) = region {
        command
            // capture region
            .arg("-g")
            .arg(region);
    }

    command
        // image format
        .args(["-t", "png"])
        // png compression level
        .args(["-l", "9"])
        // include cursor
        //.arg("-c")
        // output to stdout
        .arg("-")
        .stdout(Stdio::piped());

    let mut child = command.spawn()?;

    Command::new("wl-copy")
        .args(["-t", "image/png"])
        .stdin(child.stdout.take().unwrap())
        .spawn()?
        .wait()?;
    Ok(())
}
