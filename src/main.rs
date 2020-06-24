use std::env;
use std::io::{self, Write};
use cargo_metadata::{MetadataCommand, CargoOpt};
use tabwriter::TabWriter;

use structopt::StructOpt;

/// Extract the features for every compiled crate from `cargo metadata`.
#[derive(StructOpt, Debug)]
#[structopt(name = "cargo-feature-set")]
struct Opt {
    /// Path to Cargo.toml
    #[structopt(long)]
    manifest_path: Option<String>,

    /// Space-separated list of features to activate
    #[structopt(long)]
    features: Option<Vec<String>>,

    /// Only show root dependencies
    #[structopt(short("R"), long)]
    root_deps_only: bool,
}

fn main() {
    let mut args = env::args().collect::<Vec<_>>();
    if args.len() > 1 && args[1] == "feature-set" {
        args.drain(0..1);
    }

    let opt = Opt::from_iter(args);

    let mut cmd = MetadataCommand::new();

    if let Some(p) = opt.manifest_path {
        cmd.manifest_path(p);
    }
    if let Some(f) = opt.features {
        let features = CargoOpt::SomeFeatures(f);
        cmd.features(features);
    }

    let metadata = match cmd.exec() {
        Ok(metadata) => metadata,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };
    let resolve = metadata.resolve.unwrap();
    let packages = metadata.packages;

    let stdout = io::stdout();
    let handle = stdout.lock();
    let mut tw = TabWriter::new(handle);

    writeln!(&mut tw, "Crate\tFeatures").unwrap();
    writeln!(&mut tw, "=====\t========").unwrap();

    let mut nodes = resolve.nodes;
    if opt.root_deps_only {
        let root_node_id = resolve.root.unwrap();
        let root_node = nodes.iter().find(|n| n.id == root_node_id).unwrap().clone();
        nodes.retain(|n| root_node.dependencies.contains(&n.id))
    }
    nodes.sort_by(|left, right| left.id.cmp(&right.id));

    for node in nodes {
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
