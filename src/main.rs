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
#[serde(rename_all = "camelCase")]
struct PluginIndex {
    filename: std::string::String,

    #[serde(skip_serializing_if = "Option::is_none")]
    anti_features: Option<std::vec::Vec<std::string::String>>,
}

#[derive(Debug, serde::Serialize)]
struct AuthorIndex {
    name: std::string::String,
    plugins: std::vec::Vec<PluginIndex>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct PluginMetadata {
    #[serde(alias = "antiFeatures")]
    anti_features: Option<std::vec::Vec<std::string::String>>,
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
                    // Filter YAML files
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
                .map(|plugin_entry| {
                    // Parse YAML files and generate index
                    let filename = plugin_entry
                        .path()
                        .file_stem()
                        .expect("Unable to get file_stem of YAML file")
                        .to_os_string()
                        .into_string()
                        .expect("Unable to convert OsString to String");
                    let file = std::fs::File::open(&plugin_entry.path())
                        .expect("Unable to open YAML file");
                    let reader = std::io::BufReader::new(file);
                    let metadata: PluginMetadata =
                        serde_yaml::from_reader(reader).expect("Unable to parse YAML file");
                    return PluginIndex {
                        filename,
                        anti_features: metadata.anti_features,
                    };
                })
                .collect::<Vec<_>>();
            if plugins.is_empty() {
                return None;
            }
            return Some(AuthorIndex {
                name: author_name,
                plugins,
            });
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
