use std::io;
use std::path::Path;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

fn spawn_portal(portal: &str) -> io::Result<()> {
    let program = Path::new("/usr/libexec").join(portal);

    if let Err(error) = Command::new(program).arg("-r").spawn() {
        return Err(io::Error::other(format!(
            "Failed to spawn portal '{portal}': {error}"
        )));
    }

    sleep(Duration::from_millis(100));

    Ok(())
}

fn main() -> io::Result<()> {
    for portal in [
        "xdg-desktop-portal-wlr",
        "xdg-desktop-portal-gtk",
        "xdg-desktop-portal",
    ] {
        spawn_portal(portal)?;
    }

    Ok(())
}
