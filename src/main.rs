use shiplift::{Docker, PullOptions};
use clap::{Parser, Subcommand};
use futures::StreamExt;
// use std::env;

mod container;

/// Docker-like CLI example using clap
#[derive(Parser, Debug)]
#[command(name = "docker_cli", about = "A Docker-like CLI tool")]
struct Cli {
    #[command(subcommand)]
    command: DockerCommand,
}

#[derive(Subcommand, Debug)]
enum DockerCommand {
    /// Run a command in a new container
    Run {
        /// Image to run
        image: String,

        /// Name of the container
        #[arg(short, long)]
        name: Option<String>,

        /// Publish container's port(s) to the host
        #[arg(short = 'p', long)]
        port: Option<String>,

        /// Run container in the background and print container ID
        #[arg(short, long)]
        detach: bool,
    },

    /// Build an image from a Dockerfile
    Build {
        /// Path to the build context
        path: String,

        /// Tag name for the image
        #[arg(short, long)]
        tag: Option<String>,

        /// No cache option for the build
        #[arg(long)]
        no_cache: bool,
    },

    /// List containers
    Ps {
        /// Show all containers (default shows just running)
        #[arg(short, long)]
        all: bool,
    },

/// List images
    Images {
        /// Show all images (default hides intermediate images)                                   
        #[arg(short, long)]
        all: bool,
    },

    /// Remove one or more containers
    Rm {
        /// Force the removal of a running container
        #[arg(short, long)]
        force: bool,

        /// Container ID or name
        container: String,
    },

    /// Pull an image or a repository from a registry
    Pull {
        /// The name of the image to pull, with optional TAG or DIGEST
        #[arg(value_name = "NAME[:TAG|@DIGEST]")]
        name: String,

        /// Download all tagged images in the repository
        #[arg(short = 'a', long = "all-tags")]
        all_tags: bool,

        /// Skip image verification (default true)
        #[arg(long = "disable-content-trust", default_value = "true")]
        disable_content_trust: bool,

        /// Set platform if server is multi-platform capable
        #[arg(long = "platform")]
        platform: Option<String>,

        /// Suppress verbose output
        #[arg(short = 'q', long = "quiet")]
        quiet: bool,
    },
}


async fn images() {
    let docker = Docker::new();
    println!("docker images in stock");

    let result = docker.images().list(&Default::default()).await;

    match result {
        Ok(images) => {
            for i in images {
                println!(
                    "{} {} {:?}",
                    i.id,
                    i.created,
                    i.repo_tags.unwrap_or_else(|| vec!["none".into()])
                );
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}


async fn pull(name: String) {
    // env_logger::init();
    let docker = Docker::new();
    // let img = env::args().nth(1).expect("You need to specify an image name");
    let hub = "hub.aiursoft.cn/".to_string();
    let img = hub+&name;

    let mut stream = docker
        .images()
        .pull(&PullOptions::builder().image(img).build());

    while let Some(pull_result) = stream.next().await {
        match pull_result {
            Ok(output) => println!("{:?}", output),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}




#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        DockerCommand::Run {
            image,
            name,
            port,
            detach,
        } => {
            println!("Running container with image: {}", image);
            if let Some(name) = name {
                println!("Container name: {}", name);
            }
            if let Some(port) = port {
                println!("Port mapping: {}", port);
            }
            if *detach {
                println!("Running in detached mode");
            }
        }

        DockerCommand::Build { path, tag, no_cache } => {
            println!("Building image from path: {}", path);
            if let Some(tag) = tag {
                println!("Image tag: {}", tag);
            }
            if *no_cache {
                println!("No cache mode enabled");
            }
        }

        DockerCommand::Ps { all } => {
            if *all {
                println!("Listing all containers");
            } else {
                println!("Listing running containers");
                container::ps().await;
            }
        }
        
        DockerCommand::Images { all } => {
            if *all {
                println!("Listing all images");
            } else {
                images().await;
            }
        }

        
        // DockerCommand::container { all } => {
            // if *all {
                // println!("Listing all containers");
            // } else {
                // println!("Listing running containers");
            // }
        // }


        DockerCommand::Rm { force, container } => {
            println!("Removing container: {}", container);
            if *force {
                println!("Forcing container removal");
            }
        }

DockerCommand::Pull {
            name,
            all_tags,
            disable_content_trust,
            platform,
            quiet,
        } => {
            println!("Pulling image: {}", name);
            pull(name.to_string()).await;
            if *all_tags {
                println!("Downloading all tags...");
            }
            if !*disable_content_trust {
                println!("Image verification enabled...");
            }
            if let Some(platform) = platform {
                println!("Platform: {}", platform);
            }
            if *quiet {
                println!("Running in quiet mode...");
            }
        }
}
}