use rayon::prelude::*;
use std::fs;
use std::path::Path;

pub fn sync_files(source: &Path, destination: &Path) {
    let files: Vec<_> = std::fs
        ::read_dir(source)
        .unwrap()
        .map(|res| res.unwrap().path())
        .collect();

    files.par_iter().for_each(|file| {
        let dest_file = destination.join(file.file_name().unwrap());

        if !dest_file.exists() || is_modified(file, &dest_file) {
            println!("Syncing: {:?}", file);
            fs::copy(file, &dest_file).unwrap();
        }
    })
}

fn is_modified(source: &Path, destination: &Path) -> bool {
    let src_metadata = fs::metadata(source).unwrap();
    let dest_metadata = fs::metadata(destination).unwrap();

    src_metadata.modified().unwrap() != dest_metadata.modified().unwrap()
}
