use clap::Parser;

type FilenameVec = std::vec::Vec<std::string::String>;
type IndexMap = std::collections::HashMap<std::string::String, FilenameVec>;

fn main() {
    let arguments = Arguments::parse();

    let mut metadat_path = arguments.repository.clone();
    metadat_path.push("metadata");

    let authors = metadat_path.read_dir()
        .expect("Unable to read content of metadata")
        .filter_map(|result| result.ok());

    let mut index = IndexMap::new();

    for author in authors {
        if !author.file_type().is_ok_and(|item| item.is_dir()) {
            continue;
        }
        index.insert(
            author.path().file_stem()
                .expect("Unable to get filename")
                .to_os_string().into_string()
                .expect("Unable to convert OsString to String"), 
            author.path().read_dir().expect("Unable to read content of author")
                .filter_map(|result| result.ok())
                .filter_map(|item| {
                    if item.file_type().is_ok_and(|item| item.is_file())
                        && item.path().extension().map_or(false, |extention| extention == "yml") {
                        return Some(
                            item.path().file_stem()
                                .expect("Unable to get filename")
                                .to_os_string().into_string()
                                .expect("Unable to convert OsString to String")
                        );
                    } else {
                        return None
                    }
                })
                .collect::<Vec<_>>()
        );
    }

    if let Some(parent) = arguments.output_path.parent() {
        std::fs::create_dir_all(parent).expect("Unable to create parents");
    }
    let output = std::fs::File::create(&arguments.output_path)
        .expect("Unable to create output path");
    let writer = std::io::BufWriter::new(output);
    serde_json::to_writer(writer, &index).expect("Unable to serialize JSON");
}

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Arguments {
    #[arg(long, value_hint = clap::ValueHint::FilePath)]
    repository: std::path::PathBuf,

    #[arg(long, value_hint = clap::ValueHint::FilePath)]
    output_path: std::path::PathBuf,
}