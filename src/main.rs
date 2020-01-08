use std::io::{self, Write};
use cargo_metadata::MetadataCommand;
use tabwriter::TabWriter;

fn main() {
    let mut args = std::env::args().skip_while(|val| !val.starts_with("--manifest-path"));
    let mut cmd = MetadataCommand::new();

    match args.next() {
        Some(ref p) if p == "--manifest-path" => {
            cmd.manifest_path(args.next().unwrap());
        }
        Some(p) => {
            cmd.manifest_path(p.trim_start_matches("--manifest-path="));
        }
        None => {}
    };

    let metadata = cmd.exec().unwrap();
    let resolve = metadata.resolve.unwrap();
    let packages = metadata.packages;

    let stdout = io::stdout();
    let handle = stdout.lock();
    let mut tw = TabWriter::new(handle);

    writeln!(&mut tw, "Crate\tFeatures").unwrap();
    writeln!(&mut tw, "=====\t========").unwrap();

    for node in resolve.nodes {
        let id = node.id.clone();
        let package = match packages.iter().find(|p| p.id == id) {
            Some(package) => package,
            None => continue,
        };
        let features: Vec<_> = node.features.iter()
            .map(|f| f.to_string())
            .collect();
        let features = features.join(", ");
        writeln!(&mut tw, "{}:{}\t{}", package.name, package.version, features).ok();
    }

    tw.flush().unwrap();
}
