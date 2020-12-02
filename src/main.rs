#![windows_subsystem = "windows"]

use std::{
    env, fs, io,
    path::{Path, PathBuf},
    thread,
    time::Duration,
};

// use fs_extra;

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
            // "exe" | "msi" | "zip" => destroy_files(file),
            // "png" | "jpg" | "jpeg" | "bmp" | "gif" => copy_files(file),
            _ => destroy_files(file),
        }
    }
}

fn destroy_files(file: PathBuf) {
    fs::remove_file(file).unwrap()
}

// fn copy_files(file: PathBuf, dir: &Path) {
//     let to_docs = Path::new(&fs::read_dir("\\Documents").unwrap());
// }

fn main() {
    let dir =
        env::var("USERPROFILE").expect("Could not find USERPROFILE in environment variables.");
    let cwd = fs::read_dir(dir.clone() + "\\Documents");
    println!("{:?}", (dir.clone(), cwd));

    // run once initially before setting up the scheduler, so we can give ownership of `dir` away
    parse_files(format!("{}/{}", dir, "Downloads").as_ref());

    let mut scheduler = Scheduler::new();
    scheduler
        .every(1.day())
        .at("7:00 AM")
        .run(move || parse_files(format!("{}/{}", dir, "Downloads").as_ref()));

    loop {
        scheduler.run_pending();
        thread::sleep(Duration::from_millis(1000))
    }
}
