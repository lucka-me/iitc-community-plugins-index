use clap::Parser;

type FilenameVec = std::vec::Vec<std::string::String>;
type IndexMap = std::collections::HashMap<std::string::String, FilenameVec>;

fn main() {
    let arguments = Arguments::parse();

    let mut metadat_path = arguments.repository.clone();
    metadat_path.push("metadata");

    let authors = metadat_path
        .read_dir()
        .expect("Unable to read content of metadata")
        .filter_map(|result| {
            if let Some(entry) = result.ok() {
                if entry.file_type().is_ok_and(|file_type| file_type.is_dir()) {
                    return Some(entry);
                } else {
                    return None;
                }
            } else {
                return None;
            }
        });

    let mut index = IndexMap::new();

    for author in authors {
        index.insert(
            author
                .path()
                .file_name()
                .expect("Unable to get filename")
                .to_os_string()
                .into_string()
                .expect("Unable to convert OsString to String"),
            author
                .path()
                .read_dir()
                .expect("Unable to read content of author")
                .filter_map(|result| {
                    if let Some(entry) = result.ok() {
                        if !entry.file_type().is_ok_and(|file_type| file_type.is_file()) {
                            return None;
                        }
                        if !entry
                            .path()
                            .extension()
                            .map_or(false, |extention| extention == "yml")
                        {
                            return None;
                        }
                        if let Some(stem) = entry.path().file_stem() {
                            return Some(
                                stem.to_os_string()
                                    .into_string()
                                    .expect("Unable to convert OsString to String"),
                            );
                        } else {
                            return None;
                        }
                    } else {
                        return None;
                    }
                })
                .collect::<Vec<_>>(),
        );
    }

    if let Some(parent) = arguments.output_path.parent() {
        std::fs::create_dir_all(parent).expect("Unable to create parents");
    }
    let output =
        std::fs::File::create(&arguments.output_path).expect("Unable to create output path");
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
