mod container;
mod image;

use shiplift::{Docker, PullOptions, tty::TtyChunk};
use clap::{Parser, Subcommand, ArgAction};
use futures::StreamExt;
// use std::env;


/// Simple Rust Implementation of Docker CLI.
#[derive(Parser, Debug)]
#[command(name = "rocker", about = "Simple Rust Implementation of Docker CLI.")]
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
    
    /// Attach local standard input, output, and error streams to a running container
    Attach {
        /// The container to attach to
        #[arg(value_name = "CONTAINER")]
        container: String,

        /// Override the key sequence for detaching a container
        #[arg(long = "detach-keys")]
        detach_keys: Option<String>,

        /// Do not attach STDIN
        #[arg(long = "no-stdin")]
        no_stdin: bool,

        /// Proxy all received signals to the process (default true)
        #[arg(long = "sig-proxy", default_value = "true")]
        sig_proxy: bool,
    },
    /// Display system-wide information                                                                               
    Info {}, 
    
    /// Manage containers
    #[command(subcommand)]
    Container(container::ContainerCommand),
    
    /// Build an image from a Dockerfile
    Build {
        /// Path to the context directory or URL for the build
        #[arg(required = true)]
        path_or_url: String,

        /// Add a custom host-to-IP mapping (host:ip)
        #[arg(long, value_name = "list")]
        add_host: Option<Vec<String>>,

        /// Set build-time variables
        #[arg(long, value_name = "list")]
        build_arg: Option<Vec<String>>,

        /// Images to consider as cache sources
        #[arg(long, value_name = "strings")]
        cache_from: Option<Vec<String>>,

        /// Optional parent cgroup for the container
        #[arg(long, value_name = "string")]
        cgroup_parent: Option<String>,

        /// Compress the build context using gzip
        #[arg(long, action = ArgAction::SetTrue)]
        compress: bool,

        /// Limit the CPU CFS period
        #[arg(long, value_name = "int")]
        cpu_period: Option<u64>,

        /// Limit the CPU CFS quota
        #[arg(long, value_name = "int")]
        cpu_quota: Option<u64>,

        /// CPU shares (relative weight)
        #[arg(short = 'c', long, value_name = "int")]
        cpu_shares: Option<u64>,

        /// CPUs in which to allow execution (0-3, 0,1)
        #[arg(long, value_name = "string")]
        cpuset_cpus: Option<String>,

        /// MEMs in which to allow execution (0-3, 0,1)
        #[arg(long, value_name = "string")]
        cpuset_mems: Option<String>,

        /// Skip image verification (default true)
        #[arg(long, action = ArgAction::SetTrue)]
        disable_content_trust: bool,

        /// Name of the Dockerfile (Default is 'PATH/Dockerfile')
        #[arg(short = 'f', long, value_name = "string")]
        file: Option<String>,

        /// Always remove intermediate containers
        #[arg(long, action = ArgAction::SetTrue)]
        force_rm: bool,

        /// Write the image ID to the file
        #[arg(long, value_name = "string")]
        iidfile: Option<String>,

        /// Container isolation technology
        #[arg(long, value_name = "string")]
        isolation: Option<String>,

        /// Set metadata for an image
        #[arg(long, value_name = "list")]
        label: Option<Vec<String>>,

        /// Memory limit
        #[arg(short = 'm', long, value_name = "bytes")]
        memory: Option<String>,

        /// Swap limit equal to memory plus swap: '-1' to enable unlimited swap
        #[arg(long, value_name = "bytes")]
        memory_swap: Option<String>,

        /// Set the networking mode for the RUN instructions during build (default "default")
        #[arg(long, value_name = "string")]
        network: Option<String>,

        /// Do not use cache when building the image
        #[arg(long, action = ArgAction::SetTrue)]
        no_cache: bool,

        /// Always attempt to pull a newer version of the image
        #[arg(long, action = ArgAction::SetTrue)]
        pull: bool,

        /// Suppress the build output and print image ID on success
        #[arg(short = 'q', long, action = ArgAction::SetTrue)]
        quiet: bool,

        /// Remove intermediate containers after a successful build (default true)
        #[arg(long, action = ArgAction::SetTrue)]
        rm: bool,

        /// Security options
        #[arg(long, value_name = "strings")]
        security_opt: Option<Vec<String>>,

        /// Size of /dev/shm
        #[arg(long, value_name = "bytes")]
        shm_size: Option<String>,

        /// Name and optionally a tag in the 'name:tag' format
        #[arg(short = 't', long, value_name = "list")]
        tag: Option<Vec<String>>,

        /// Set the target build stage to build
        #[arg(long, value_name = "string")]
        target: Option<String>,

        /// Ulimit options
        #[arg(long, value_name = "ulimit")]
        ulimit: Option<Vec<String>>,
    },
    
/// Remove one or more images
    Rmi {
        
        /// image to delete
        #[arg(required = true)]
        image: [&str],
        
        /// Force removal of the image                                                                 
        #[arg(short, long)]
        force: bool,
        
        /// Do not delete untagged parents
        #[arg(long)]
        no_prune: bool,
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
    // cndocker 
    let img = hub+&name;
    // rocker
    // let img = name;

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

async fn attach(id: String) -> Result<(), Box<dyn std::error::Error>> {
    let docker = Docker::new();
    // let id = env::args().nth(1).expect("You need to specify a container id");


    let tty_multiplexer = docker.containers().get(&id).attach().await?;

    let (mut reader, _writer) = tty_multiplexer.split();

    while let Some(tty_result) = reader.next().await {
        match tty_result {
            Ok(chunk) => print_chunk(chunk),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    Ok(())
}

fn print_chunk(chunk: TtyChunk) {
    match chunk {
        TtyChunk::StdOut(bytes) => println!("Stdout: {}", std::str::from_utf8(&bytes).unwrap()),
        TtyChunk::StdErr(bytes) => eprintln!("Stdout: {}", std::str::from_utf8(&bytes).unwrap()),
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

        DockerCommand::Build {
            path_or_url,
            add_host,
            build_arg,
            cache_from,
            cgroup_parent,
            compress,
            cpu_period,
            cpu_quota,
            cpu_shares,
            cpuset_cpus,
            cpuset_mems,
            disable_content_trust,
            file,
            force_rm,
            iidfile,
            isolation,
            label,
            memory,
            memory_swap,
            network,
            no_cache,
            pull,
            quiet,
            rm,
            security_opt,
            shm_size,
            tag,
            target,
            ulimit,
        } => {
            // 处理 `docker build` 的逻辑
            println!("Building image from path or URL: {}", path_or_url);
            image::build(path_or_url).await;

            if let Some(add_hosts) = add_host {
                for host in add_hosts {
                    println!("Adding custom host: {}", host);
                }
            }

            if let Some(args) = build_arg {
                for arg in args {
                    println!("Using build argument: {}", arg);
                }
            }

            if *compress {
                println!("Compressing the build context.");
            }

            if * no_cache {
                println!("Disabling cache during build.");
            }

            if let Some(tags) = tag {
                for t in tags {
                    println!("Tagging image as: {}", t);
                }
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

        DockerCommand::Container(container_command) => {
            container::handle_container_command(container_command).await;
        }

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
        
        DockerCommand::Attach {
            container,
            detach_keys,
            no_stdin,
            sig_proxy,
        } => {
            println!("Attaching to container: {}", container);
            match attach(container.to_string()).await {
    Ok(_) => println!("Attached to container"),
    Err(e) => eprintln!("Failed to attach: {}", e),
}
            if let Some(keys) = detach_keys {
                println!("Detach keys override: {}", keys);
            }
            if *no_stdin {
                println!("STDIN will not be attached.");
            }
            if *sig_proxy {
                // println!("Signal proxying is enabled.");
            } else {
                println!("Signal proxying is disabled.");
            }
        }
        
        DockerCommand::Info {} => {
            info().await;
        }

DockerCommand::Rmi {image,  force, no_prune } => {
        // 处理 rmi 命令
        image::rmi(image).await;
        if *force {
            println!("Force removal of the image.");
        }
        if *no_prune {
            println!("Do not delete untagged parents.");
        }
    }
}
}