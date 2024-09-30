mod container;
mod image;
mod remote;

use shiplift::{
    Docker,
    tty::TtyChunk
};
use clap::{
    Parser,
    Subcommand
};
use futures::StreamExt;
// use std::env;


/// Simple Rust Implementation of Docker CLI.
 # [derive(Parser, Debug)]
 # [command(name = "rocker", about = "Simple Rust Implementation of Docker CLI.")]
struct Cli {
     # [command(subcommand)]
    command: DockerCommand,
}

 # [derive(Subcommand, Debug)]
enum DockerCommand {
    /// Run a command in a new container
    Run {
        /// Image to run
        image: String,

        /// Command  to run
        command: Option < String > ,

        /// Arguments of command
        arguments: Option < Vec < String >> ,

        /// run options
         # [clap(flatten)]
        options: container::RunArgOptions,

    },

    /// List containers
    Ps {
        /// Show all containers (default shows just running)
         # [arg(short, long)]
        all: bool,
    },

    /// List images
    Images {
        /// Show all images (default hides intermediate images)
         # [arg(short, long)]
        all: bool,
    },

    /// Remove one or more containers
    Rm {
        /// Container ID or name
        container: String,

        /// Force the removal of a running container
         # [arg(short, long)]
        force: bool,

        /// Remove the specified link
         # [arg(short, long)]
        link: Option < String > ,

        /// Remove anonymous volumes associated with the container
         # [arg(short, long)]
        volumes: Option < String > ,
    },

    /// Pull an image or a repository from a registry
    Pull {
        /// The name of the image to pull, with optional TAG or DIGEST
         # [arg(value_name = "NAME[:TAG|@DIGEST]")]
        name: String,

        /// Download all tagged images in the repository
         # [arg(short = 'a', long = "all-tags")]
        all_tags: bool,

        /// Skip image verification (default true)
         # [arg(long = "disable-content-trust", default_value = "true")]
        disable_content_trust: bool,

        /// Set platform if server is multi-platform capable
         # [arg(long = "platform")]
        platform: Option < String > ,

        /// Suppress verbose output
         # [arg(short = 'q', long = "quiet")]
        quiet: bool,
    },

    /// Attach local standard input, output, and error streams to a running container
    Attach {
        /// The container to attach to
         # [arg(value_name = "CONTAINER")]
        container: String,

        /// Override the key sequence for detaching a container
         # [arg(long = "detach-keys")]
        detach_keys: Option < String > ,

        /// Do not attach STDIN
         # [arg(long = "no-stdin")]
        no_stdin: bool,

        /// Proxy all received signals to the process (default true)
         # [arg(long = "sig-proxy", default_value = "true")]
        sig_proxy: bool,
    },
    /// Display system-wide information
    Info {},

    /// Manage containers
     # [command(subcommand)]
    Container(container::ContainerCommand),

    /// Manage images
     # [command(subcommand)]
    Image(image::ImageCommand),

    /// Build an image from a Dockerfile
    Build {
        /// Path to the context directory or URL for the build
         # [arg(required = true)]
        path_or_url: String,

        /// Build options
         # [clap(flatten)]
        options: image::BuildArgOptions,
    },

    /// Remove one or more images
    Rmi {
        /// image to delete
         # [arg(required = true)]
        image: String,

        /// Force removal of the image
         # [arg(short, long)]
        force: bool,

        /// Do not delete untagged parents
         # [arg(long)]
        no_prune: bool,
    },

    /// Run  remote command
     # [command(subcommand)]
    Remote(remote::RemoteCommand),
}

async fn images() {
    let docker = Docker::new();
    println!("docker images in stock");

    let result = docker.images().list( & Default::default ()).await;

            match result {
                Ok(images) => {
                    for i in images {
                        println!(
                            "{} {} {:?}",
                            i.id,
                            i.created,
                            i.repo_tags.unwrap_or_else( || vec!["none".into()]));
                    }
                }
                Err(e) => eprintln!("Error: {}", e),
            }
}

async fn attach(id: String)->Result < (), Box < dyn std::error::Error >> {
    let docker = Docker::new();
    // let id = env::args().nth(1).expect("You need to specify a container id");


    let tty_multiplexer = docker.containers().get( & id).attach().await ? ;

    let(mut reader, _writer) = tty_multiplexer.split();
    while let Some(tty_result) = reader.next().await {
        match tty_result {
            Ok(chunk) => print_chunk(chunk),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    Ok(())
}

fn print_chunk(chunk : TtyChunk) {
    match chunk {
        TtyChunk::StdOut(bytes) => println!("Stdout: {}", std::str::from_utf8( & bytes).unwrap()),
        TtyChunk::StdErr(bytes) => eprintln!("Stdout: {}", std::str::from_utf8( & bytes).unwrap()),
        TtyChunk::StdIn(_) => unreachable!(),
    }
}

async fn info() {
    let docker = Docker::new();

    match docker.info().await {
        Ok(info) => println!("info {:?}", info),
        Err(e) => eprintln!("Error: {}", e),
    }
}

 # [tokio::main]
async fn main() {
    let cli = Cli::parse();

    match & cli.command {
        DockerCommand::Run {
            image,
            command,
            arguments,
            options,
        }
         => {
            println!("Running container with image: {}", image);
            container::create(image, options.clone()).await;
        }

        DockerCommand::Build {
            path_or_url,
            options
        }
         => {
            // 处理 `docker build` 的逻辑
            println!("Building image from path or URL: {}", path_or_url);
            let cloned_options = options.clone();
            image::build(path_or_url, cloned_options).await;

        }

        DockerCommand::Ps {
            all
        }
         => {
            println!("Listing containers");
            container::ps(all.clone()).await;
        }
    DockerCommand::Images {
        all
    }
     => {
        if  * all {
            println!("Listing all images");
        }
    else {
        images().await;
    }
}

DockerCommand::Container(container_command) => {
    container::handle_container_command(container_command).await;
}

DockerCommand::Image(image_command) => {
    image::handle_image_command(image_command).await;
}

DockerCommand::Rm {
    container,
    force,
    volumes,
    link
}
 => {
    println!("Removing container: {}", container);
    if  * force {
        println!("Forcing container removal");
    }
if let Some(somevolumes) = volumes {
    println!("volumes:{}", somevolumes);
}
if let Some(link) = link {
    println!("link:{}", link);
}

}

DockerCommand::Pull {
    name,
    all_tags,
    disable_content_trust,
    platform,
    quiet,
}
 => {
    println!("Pulling image: {}", name);
    image::pull(name.to_string()).await;
    if  * all_tags {
        println!("Downloading all tags...");
    }
if ! * disable_content_trust {
    println!("Image verification enabled...");
}
if let Some(platform) = platform {
    println!("Platform: {}", platform);
}
if  * quiet {
    println!("Running in quiet mode...");
}
}

DockerCommand::Attach {
    container,
    detach_keys,
    no_stdin,
    sig_proxy,
}
 => {
    println!("Attaching to container: {}", container);
    match attach(container.to_string()).await {
        Ok(_) => println!("Attached to container"),
        Err(e) => eprintln!("Failed to attach: {}", e),
    }
    if let Some(keys) = detach_keys {
        println!("Detach keys override: {}", keys);
    }
    if  * no_stdin {
        println!("STDIN will not be attached.");
    }
if  * sig_proxy {
    // println!("Signal proxying is enabled.");
}
else {
    println!("Signal proxying is disabled.");
}
}

DockerCommand::Info {}
 => {
    info().await;
}

DockerCommand::Rmi {
    image,
    force,
    no_prune
}
 => {
    // 处理 rmi 命令
    image::rmi(image.to_string()).await;
    if  * force {
        println!("Force removal of the image.");
    }
if  * no_prune {
    println!("Do not delete untagged parents.");
}
}

DockerCommand::Remote(remote_command) => {
    // println!("executing  command");
    remote::handle_remote_command(remote_command).await;
}
}
}
