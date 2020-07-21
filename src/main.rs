#![windows_subsystem = "windows"]

use std::{
    env, fs, io,
    path::{Path, PathBuf},
    thread,
    time::Duration,
};

use clokwerk::{Scheduler, TimeUnits};

fn get_files(dir: &Path) -> Result<Vec<PathBuf>, io::Error> {
    Ok(fs::read_dir(dir)?
        .into_iter()
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap().path())
        .filter(|r| r.is_file())
        .collect())
}

fn parse_files(dir: &Path) {
    let files = get_files(dir.as_ref()).expect("Error getting files.");

    for file in files {
        let ext = file
            .extension()
            .expect("Could not read file extension.")
            .to_str()
            .unwrap();

        match ext {
            "exe" => destroy(file),
            "msi" => destroy(file),
            "zip" => destroy(file),
            _ => {}
        }
    }
}

fn destroy(file: PathBuf) {
    fs::remove_file(file).unwrap()
}

fn main() {
    let dir =
        env::var("USERPROFILE").expect("Could not find USERPROFILE in environment variables.");

    let mut scheduler = Scheduler::new();
    scheduler
        .every(7.days())
        .run(move || parse_files(format!("{}/{}", dir, "Downloads").as_ref()));

    loop {
        scheduler.run_pending();
        thread::sleep(Duration::from_secs(3600));
    }
}
