use std::env;

use azure_devops_rust_api::{git, Credential};
use clap::Parser;
use futures::executor::block_on;
use git2::Repository;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    pat: String,

    #[arg(short, long)]
    organization: String,

    #[arg(short, long)]
    project: String,

    #[arg(short, long)]
    url: String,
}

fn main() {
    let args = Args::parse();

    block_on(git(args))
}

async fn git(args: Args) {
    let credential = Credential::from_pat(&args.pat);
    let git_client = git::ClientBuilder::new(credential).build();

    let repos = git_client
        .repositories_client()
        .list(&args.organization, &args.project)
        .await
        .unwrap();

    for repo in repos.value {
        println!("Cloning {}", repo.name);
        let path = match env::current_dir() {
            Ok(path) => path,
            Err(e) => panic!("{}", e),
        };
        _ = match Repository::clone(&repo.url, path) {
            Ok(cloned) => cloned,
            Err(e) => panic!("Failed to clone {}", e),
        };
    }
}
