use clap::Parser;

#[derive(Debug, clap::Parser)]
#[command(author, version, about)]
struct Arguments {
    #[arg(long, value_hint = clap::ValueHint::FilePath)]
    repository: std::path::PathBuf,

    #[arg(long, value_hint = clap::ValueHint::FilePath)]
    output_path: std::path::PathBuf,
}

#[derive(Debug, serde::Serialize)]
struct PluginMetadata {
    filename: std::string::String,
}

#[derive(Debug, serde::Serialize)]
struct Author {
    name: std::string::String,
    plugins: std::vec::Vec<PluginMetadata>,
}

fn main() {
    let arguments = Arguments::parse();

    let mut metadata_path = arguments.repository.clone();
    metadata_path.push("metadata");

    let index = metadata_path
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
        })
        .filter_map(|author_entry| {
            let author_path = author_entry.path();
            let author_name = author_path
                .file_name()
                .expect("Unable to get filename")
                .to_os_string()
                .into_string()
                .expect("Unable to convert OsString to String");
            let plugins = author_path
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
                        return Some(entry);
                    } else {
                        return None;
                    }
                })
                .filter_map(|plugin_entry| {
                    if let Some(stem) = plugin_entry.path().file_stem() {
                        return Some(PluginMetadata {
                            filename: stem
                                .to_os_string()
                                .into_string()
                                .expect("Unable to convert OsString to String"),
                        });
                    } else {
                        return None;
                    }
                })
                .collect::<Vec<_>>();
            if plugins.is_empty() {
                return None;
            }
            return Some(Author { name: author_name, plugins });
        })
        .collect::<Vec<_>>();

    if let Some(parent) = arguments.output_path.parent() {
        std::fs::create_dir_all(parent).expect("Unable to create parents");
    }
    let output =
        std::fs::File::create(&arguments.output_path).expect("Unable to create output path");
    let writer = std::io::BufWriter::new(output);
    serde_json::to_writer(writer, &index).expect("Unable to serialize JSON");
}
