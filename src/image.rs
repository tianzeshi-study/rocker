use futures::StreamExt;
use shiplift::{
    PullOptions,
    BuildOptions,
    Docker,
    builder::BuildOptionsBuilder,
};

use clap::{
    Parser,
    // ArgAction
    Subcommand,
};

 # [derive(Subcommand, Debug)]
pub enum ImageCommand {
    ///Build an image from a Dockerfile
    Build {},
    ///       Show the history of an image
    History {},
    ///  Import the contents from a tarball to create a filesystem image
    Import {},
    ///       Display detailed information on one or more images
    Inspect {},
    ///          Load an image from a tar archive or STDIN
    Load {},
    ///           List images
    Ls {},
    ///        Remove unused images
    Prune {},
    ///          Pull an image or a repository from a registry
    Pull {},
    ///         Push an image or a repository to a registry
    Push {},
    ///           Remove one or more images
    Rm {},
    /// Save one or more images to a tar archive (streamed to STDOUT by default)
    Save {},
    /// Create a tag TARGET_IMAGE that refers to SOURCE_IMAGE
    Tag {},
}

pub async fn handle_image_command(command:  & ImageCommand) {
    /*
    match command {
    ContainerCommand::Attach {
    container,
    detach_keys,
    no_stdin,
    sig_proxy,
    }
    => {
    println!(
    "Attaching to container: {} (detach_keys: {:?}, no_stdin: {}, sig_proxy: {})",
    container, detach_keys, no_stdin, sig_proxy);
    // 在这里处理 Attach 逻辑
    }
    ContainerCommand::Commit {
    container,
    repository,
    message,
    }
    => {
    println!(
    "Committing container: {} (repository: {:?}, message: {:?})",
    container, repository, message);
    // 在这里处理 Commit 逻辑
    }
    ContainerCommand::Cp {
    src,
    dest
    }
    => {
    println!("Copying from {} to {}", src, dest);
    // 在这里处理 Cp 逻辑
    }
    ContainerCommand::Create {
    image,
    command,
    arguments,
    options,
    } => {
    println!("Running container with image: {}", image);
    create(image, options.clone()).await;
    }

    ContainerCommand::Diff {
    container
    }
    => {
    println!("Inspecting changes to container: {}", container);
    // 在这里处理 Diff 逻辑
    }
    ContainerCommand::Exec {
    container,
    command
    }
    => {
    println!("Executing command '{}' in container: {}", command, container);
    // 在这里处理 Exec 逻辑
    }
    ContainerCommand::Logs {
    container,
    }
    => {
    println!("Get logs of container: {}", container);
    logs(container.to_string()).await;
    }
    // 可以继续添加其他命令的处理逻辑
    ContainerCommand::Export {
    container,
    }
    => {
    // println!("exporting container '{:?}' : {}", command, container);
    export(container.to_string()).await;
    }

    _ => println!("Command not implemented yet."),
    }
     */
}

 # [derive(Parser, Debug, Clone)]
pub struct BuildArgOptions {
    /*
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
     */
    /// Name and optionally a tag in the 'name:tag' format
     # [arg(short = 't', long, value_name = "list")]
    tag: Option < String > ,
    // tag: Option < Vec < String >> ,
    /*
    /// Set the target build stage to build
    #[arg(long, value_name = "string")]
    target: Option<String>,

    /// Ulimit options
    #[arg(long, value_name = "ulimit")]
    ulimit: Option<Vec<String>>,
     */
}

pub async fn build(path:  & str, build_options: BuildArgOptions) {
    let docker = Docker::new();
    // let path = env::args().nth(1).expect("You need to specify a path");

    let mut options:  & mut BuildOptionsBuilder =  & mut BuildOptions::builder(path);
    // let mut options = BuildOptionsBuilder::new(path);
    if let Some(tag) = build_options.tag {
        // let options = BuildOptions::builder(path).tag("shiplift_test").build();
        // let options = BuildOptions::builder(path).tag(tag).build();
        options = options.tag(tag);
    }
    else {}
    let final_options: BuildOptions = options.build();

    let mut stream = docker.images().build( & final_options);
    while let Some(build_result) = stream.next().await {
        match build_result {
            Ok(output) => println!("{:?}", output),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

pub async fn rmi(image: String) {
    let docker = Docker::new();
    // let img = env::args().nth(1).expect("You need to specify an image name");

    let img = image;
    match docker.images().get( & img).delete().await {
        Ok(statuses) => {
            for status in statuses {
                println!("{:?}", status);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

pub async fn pull(name: String) {
    // env_logger::init();
    let docker = Docker::new();
    // let img = env::args().nth(1).expect("You need to specify an image name");
    let hub = "hub.aiursoft.cn/".to_string();
    // cndocker
    let img = hub +  & name;
    // rocker
    // let img = name;

    let mut stream = docker
        .images()
        .pull( & PullOptions::builder().image(img).build());
    while let Some(pull_result) = stream.next().await {
        match pull_result {
            Ok(output) => println!("{:?}", output),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
