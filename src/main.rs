use clap::{Parser, Subcommand};

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

    /// Remove one or more containers
    Rm {
        /// Force the removal of a running container
        #[arg(short, long)]
        force: bool,

        /// Container ID or name
        container: String,
    },
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
    }
}