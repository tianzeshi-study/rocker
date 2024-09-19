use shiplift::{
    Docker,
    ContainerListOptions
};
use clap::{
    // Parser,
    Subcommand
};

 # [derive(Subcommand, Debug)]
pub enum ContainerCommand {
    /// Attach local standard input, output, and error streams to a running container
    Attach {
        /// Container name or ID
        container: String,

        /// Override the key sequence for detaching a container
         # [arg(long, value_name = "string")]
        detach_keys: Option < String > ,

        /// Do not attach STDIN
         # [arg(long)]
        no_stdin: bool,

        /// Proxy all received signals to the process (default true)
         # [arg(long)]
        sig_proxy: bool,
    },

    /// Create a new image from a container's changes
    Commit {
        /// Container name or ID
        container: String,

        /// Repository name for new image
        repository: Option < String > ,

        /// Commit message
         # [arg(long)]
        message: Option < String > ,
    },

    /// Copy files/folders between a container and the local filesystem
    Cp {
        /// Source path
        src: String,

        /// Destination path
        dest: String,
    },

    /// Create a new container
    Create {
        /// Image to create the container from
        image: String,

        /// Name of the container
         # [arg(short, long)]
        name: Option < String > ,

        /// Publish container's port(s) to the host
         # [arg(short = 'p', long)]
        port: Option < String > ,
    },

    /// Inspect changes to files or directories on a container's filesystem
    Diff {
        /// Container name or ID
        container: String,
    },

    /// Run a command in a running container
    Exec {
        /// Container name or ID
        container: String,

        /// Command to execute
        command: String,
    },

    /// Export a container's filesystem as a tar archive
    Export {
        /// Container name or ID
        container: String,
    },

    /// Display detailed information on one or more containers
    Inspect {
        /// Container name or ID
        container: String,
    },

    /// Kill one or more running containers
    Kill {
        /// Container name or ID
        container: String,
    },

    /// Fetch the logs of a container
    Logs {
        /// Container name or ID
        container: String,
    },

    /// List containers
    Ls,

    /// Pause all processes within one or more containers
    Pause {
        /// Container name or ID
        container: String,
    },

    /// List port mappings or a specific mapping for the container
    Port {
        /// Container name or ID
        container: String,
    },

    /// Remove all stopped containers
    Prune,

    /// Rename a container
    Rename {
        /// Current container name or ID
        container: String,

        /// New container name
        new_name: String,
    },

    /// Restart one or more containers
    Restart {
        /// Container name or ID
        container: String,
    },

    /// Remove one or more containers
    Rm {
        /// Container name or ID
        container: String,
    },

    /// Run a command in a new container
    Run {
        /// Image to run
        image: String,

        /// Name of the container
         # [arg(short, long)]
        name: Option < String > ,

        /// Publish container's port(s) to the host
         # [arg(short = 'p', long)]
        port: Option < String > ,

        /// Run container in the background and print container ID
         # [arg(short, long)]
        detach: bool,
    },

    /// Start one or more stopped containers
    Start {
        /// Container name or ID
        container: String,
    },

    /// Display a live stream of container(s) resource usage statistics
    Stats {
        /// Container name or ID
        container: String,
    },

    /// Stop one or more running containers
    Stop {
        /// Container name or ID
        container: String,
    },

    /// Display the running processes of a container
    Top {
        /// Container name or ID
        container: String,
    },

    /// Unpause all processes within one or more containers
    Unpause {
        /// Container name or ID
        container: String,
    },

    /// Update configuration of one or more containers
    Update {
        /// Container name or ID
        container: String,
    },

    /// Block until one or more containers stop, then print their exit codes
    Wait {
        /// Container name or ID
        container: String,
    },
}

pub async fn handle_container_command(command: &ContainerCommand) {
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
                name,
                port
            }
             => {
                println!(
                    "Creating container from image: {} (name: {:?}, port: {:?})",
                    image, name, port);
                // 在这里处理 Create 逻辑
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
            // 可以继续添加其他命令的处理逻辑
            _ => println!("Command not implemented yet."),
    }
}

pub async fn ps() {
    // env_logger::init();
    let docker = Docker::new();
    match docker.containers().list( & ContainerListOptions::default ()).await {
            Ok(containers) => {
                for c in containers {
                    println!("container -> {:#?}", c)
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
