#[allow(dead_code)]
mod reader;

#[cfg(test)]
mod tests {
    use super::reader::{Reader};
    use std::env;
    use std::fs;
    use std::io;
    use std::fs::{File};
    use std::path::{PathBuf};
    use zip;

    #[test]
    fn it_works() {
        let repo_name = "TestRepo01";
        let mut repo_dir = env::temp_dir();
        repo_dir.push(repo_name);
        if repo_dir.exists() {
            fs::remove_dir_all(&repo_dir).expect("Unable to delete dir");
        }

        let mut zip_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        zip_path.push("resources");
        zip_path.push(repo_name.to_owned() + ".zip");

        let mut archive = zip::ZipArchive::new(File::open(zip_path).unwrap()).unwrap();

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            let outpath = repo_dir.join(file.sanitized_name());
            
            if file.name().ends_with('/') {
                fs::create_dir_all(&outpath).unwrap();
            }
            else {
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        fs::create_dir_all(&p).unwrap();
                    }
                }
                let mut outfile = fs::File::create(&outpath).unwrap();
                io::copy(&mut file, &mut outfile).unwrap();
            }
        }

        let reader = Reader{ repository_path: repo_dir.to_str().unwrap().to_owned() };
        let res = reader.read_loose_file("d670460b4b4aece5915caf5c68d12f560a9fe3e4");
        assert_eq!(res, "test content\n");
    }
}