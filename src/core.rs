use std::ffi::OsStr;
use std::io;
use std::fs::File;
use std::io::BufWriter;
use std::path::Component;
use zip::ZipWriter;
use zip::read::ZipFile;
use zip::write::FileOptions;
use zip::result::ZipResult;
use zip::CompressionMethod;
use zip::ZipArchive;
use walkdir::WalkDir;
use std::io::{Write, BufReader, BufRead};
use std::path::Path;

const WORKSPACE_LABEL: &str = "workspace";

fn file_bytes(f: File) -> Vec<u8> {
	let file_length = f.metadata().unwrap().len() as usize;
	let mut reader = BufReader::with_capacity(file_length, f);
	reader.fill_buf().unwrap().into()
}

fn convert_path_to_relative(parent_path: &String, absolute_path: &Path) -> String {
    let path_string = absolute_path.to_str().unwrap();
    let relative_path = &path_string[parent_path.len() + 1..path_string.len()];
    String::from(relative_path)
}

fn crawl_directory_and_append_to_archive(
    root_dir: &String,
    parent_dir: String, 
    archive_builder: &mut ZipWriter<File>,
    options: FileOptions
) {
    WalkDir::new(&parent_dir)
        .min_depth(1)
        .max_depth(1)
        .contents_first(true)
        .into_iter()
        .map(|result| result.unwrap())
        .for_each(|entry| {
            let path = entry.path();
            let path_string = path.to_str().unwrap().to_string();

            if path.is_dir() {
                crawl_directory_and_append_to_archive(root_dir, path_string, archive_builder, options);
                return;
            }

            let file = File::open(path)
                .map_err(|_| println!("Errored file: {}", path_string))
                .unwrap();

            archive_builder
                .start_file(convert_path_to_relative(&root_dir, path), options)
                .unwrap();

            let _size = archive_builder
                .write(&file_bytes(file))
                .unwrap();
        });
}



fn create_directories_for_extraction(
    destination_folder: &String, 
    file_path: &String, 
    path_seperator: char
) -> io::Result<String> {
    let destination_folder_path = Path::new(destination_folder);
    if !destination_folder_path.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Parent directory does not exist".to_string()));
    }

    let position_of_last_path_seperator =  file_path.chars().rev().position(|x| x == path_seperator).unwrap_or(0);


    if position_of_last_path_seperator == 0 {
        return Ok(format!("{}\\{}", destination_folder, &file_path));
    } 

    //TODO(teddy) possible bug, get the directory of the file_path
    let full_file_path = format!(
        "{}{}{}", 
        destination_folder, 
        path_seperator.to_owned(), 
        &file_path[0..file_path.len() - position_of_last_path_seperator],
        );
    let new_path = Path::new(&full_file_path);

    println!("Reached here");
    if !new_path.exists() { std::fs::create_dir_all(new_path)?; }

    Ok(format!("{}{}", full_file_path, &file_path[file_path.len() - position_of_last_path_seperator..file_path.len()]))
}

struct Workspace {
	path: String,
    path_to_backup: String,
    extraction_path: String,
}


impl Workspace {
	pub fn new(path: String, path_to_backup: String, extraction_path: String) -> Self {
		Self {
			path,
            path_to_backup,
            extraction_path
		}
	}

	pub fn archive(&self) -> ZipResult<()> {
		//TODO(teddy) Make it password locked
		//Implement some sort of progress

		let archive_file = File::create(&self.path_to_backup).unwrap();
		let mut archive_builder = ZipWriter::new(archive_file);
		let options = FileOptions::default()
			.compression_method(CompressionMethod::Stored);

        crawl_directory_and_append_to_archive(&self.path.clone(), self.path.clone(), &mut archive_builder, options);
		match archive_builder.finish() {
			Ok(_) => Ok(()),
			Err(err) => Err(err)
		}
	}

	pub fn unarchive(&self) -> ZipResult<()> {
		//Decompress and save files to filesytem
        let file = File::open(&self.path_to_backup).unwrap();
        let mut archive = ZipArchive::new(file)?;

        for i in 0..archive.len() {
            let mut archived_file: ZipFile = archive.by_index(i).unwrap();
            let archived_file_size = archived_file.size() as usize;

            let extraction_path = if cfg!(windows) {
                create_directories_for_extraction(&self.extraction_path, &archived_file.name().to_string(), '\\')
            } else if cfg!(unix) {
                create_directories_for_extraction(&self.extraction_path, &archived_file.name().to_string(), '/')
            } else {
                panic!("Your platform is not supported yet")
            };

            let output_file = File::create(extraction_path.unwrap()).unwrap();

            let mut buffered_writer = BufWriter::with_capacity(archived_file_size, output_file);
            let bytes_copied = std::io::copy(&mut archived_file, &mut buffered_writer).unwrap() as usize;

            assert_eq!(bytes_copied, archived_file_size, "Expecting {} from zip to be equal to {} ", bytes_copied, archived_file_size);
            buffered_writer
                .flush()
                .unwrap();

            println!("extracted_entries: {}", archived_file.name());
        }

        Ok(())
	}
}


mod test {
    use super::{Workspace, create_directories_for_extraction};
    use std::path::Path;

	#[test]
	fn make_archive() {
        let source = String::from("C:\\Users\\teddj\\workspace\\test");
        let destination = String::from("C:\\Users\\teddj\\workspace\\backup.zip");
        let extraction_path = String::from("C:\\Users\\teddj\\workspace\\test_extract");

		let workspace = Workspace::new(source, destination, extraction_path);
		workspace.archive().unwrap();

        workspace.unarchive()
            .unwrap();
	}

    #[test]
    fn test_create_directories_for_extraction() {
        let path = create_directories_for_extraction(&"C:\\Users\\teddj\\workspace".to_string(), &"helloworld.jpg".to_string(), '\\')
            .unwrap();

        let expected_out_directory_name = format!("C:\\Users\\teddj\\workspace\\{}", "helloworld.jpg");
        assert_eq!(path, expected_out_directory_name);
    }
}
