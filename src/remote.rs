use reqwest::Client;
use serde::{
    Serialize,
    Deserialize
};
use std::error::Error;
use std::fs;
// use std::io::Result;
use clap::{
    Parser,
    Subcommand
};

 # [derive(Subcommand, Debug)]
pub enum RemoteCommand {
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
        /// Image to run
        image: String,

        /// Command  to run
        command: Option < String > ,

        /// Arguments of command
        arguments: Option < Vec < String >> ,

        /// run options
         # [clap(flatten)]
        options: RunArgOptions,

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
        command: Option < String > ,

        ///  Single file to  read
         # [arg(short, long)]
        file_path: Option < String > ,
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

        /// Command  to run
        command: Option < String > ,

        /// Arguments of command
        arguments: Option < Vec < String >> ,

        /// run options
         # [clap(flatten)]
        options: RunArgOptions,

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

pub async fn handle_remote_command(command:  & RemoteCommand) {
    match command {
        RemoteCommand::Attach {
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
        RemoteCommand::Commit {
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
        RemoteCommand::Cp {
            src,
            dest
        }
         => {
            println!("Copying from {} to {}", src, dest);
            // 在这里处理 Cp 逻辑
        }
        RemoteCommand::Create {
            image,
            command,
            arguments,
            options,
        }
         => {
            println!("Running container with image: {}", image);
            // create(image, options.clone()).await;
        }

        RemoteCommand::Diff {
            container
        }
         => {
            println!("Inspecting changes to container: {}", container);
            // 在这里处理 Diff 逻辑
        }
        RemoteCommand::Exec {
            container,
            command,
            file_path,
        }
         => {
            if let Some(file_path) = file_path {
                let file_content = fs::read_to_string(file_path).unwrap();
                println!("executing {} in container", file_path);
                match exec(container.to_string(), file_content).await {
                    Ok(()) => println!("ok"),
                    Err(e) => eprintln!("{}", e),
                }

            }
            else {
                if let Some(command) = command.clone() {
                    println!("Executing command '{}' in container: {}", command, container);
                    // 在这里处理 Exec 逻辑
                    // let _ =exec(container.to_string(),command.to_string()).await;
                    // let _ =exec(container.to_string(),command.to_string(), file_path.clone()).await;
                    // match  exec(container.to_string(),command.to_string(), file_path.clone()).await {
                    match exec(container.to_string(), command.to_string()).await {
                        Ok(()) => println!("ok"),
                        Err(e) => eprintln!("{}", e),
                    }
                }
                else {
                    println!("command or code needed!");
                }
            }
        }
        RemoteCommand::Ls => ls().await,
        // 可以继续添加其他命令的处理逻辑


        _ => println!("Command not implemented yet."),
    }
}

 # [derive(Parser, Debug, Clone)]
pub struct RunArgOptions {
    /// Add a custom host-to-IP mapping (host:ip)
     # [arg(long, value_name = "list")]
    add_host: Option < Vec < String >> ,

    /// Attach to STDIN, STDOUT or STDERR
     # [arg(short = 'a', long, value_name = "list")]
    attach: Option < Vec < String >> ,

    /// Block IO weight between 10 and 1000, or 0 to disable
     # [arg(long, value_name = "uint16")]
    blkio_weight: Option < u16 > ,

    /// Block IO weight for a device
     # [arg(long, value_name = "list")]
    blkio_weight_device: Option < Vec < String >> ,

    /// Add Linux capabilities
     # [arg(long, value_name = "list")]
    cap_add: Option < Vec < String >> ,

    /// Drop Linux capabilities
     # [arg(long, value_name = "list")]
    cap_drop: Option < Vec < String >> ,

    /// Optional parent cgroup for the container
     # [arg(long)]
    cgroup_parent: Option < String > ,

    /// Cgroup namespace to use (host|private)
     # [arg(long)]
    cgroupns: Option < String > ,

    /// Write the container ID to the file
     # [arg(long)]
    cidfile: Option < String > ,

    /// Limit CPU CFS (Completely Fair Scheduler) period
     # [arg(long)]
    cpu_period: Option < i64 > ,

    /// Limit CPU CFS quota
     # [arg(long)]
    cpu_quota: Option < i64 > ,

    /// Limit CPU real-time period in microseconds
     # [arg(long)]
    cpu_rt_period: Option < i64 > ,

    /// Limit CPU real-time runtime in microseconds
     # [arg(long)]
    cpu_rt_runtime: Option < i64 > ,

    /// CPU shares (relative weight)
     # [arg(short = 'c', long)]
    cpu_shares: Option < i64 > ,

    /// Number of CPUs
     # [arg(long, value_name = "decimal")]
    cpus: Option < f64 > ,

    /// CPUs in which to allow execution (0-3, 0,1)
     # [arg(long)]
    cpuset_cpus: Option < String > ,

    /// MEMs in which to allow execution (0-3, 0,1)
     # [arg(long)]
    cpuset_mems: Option < String > ,

    /// Run container in background and print container ID
     # [arg(short = 'd', long)]
    detach: bool,

    /// Override the key sequence for detaching a container
     # [arg(long)]
    detach_keys: Option < String > ,

    /// Add a host device to the container
     # [arg(long, value_name = "list")]
    device: Option < Vec < String >> ,

    /// Add a rule to the cgroup allowed devices list
     # [arg(long, value_name = "list")]
    device_cgroup_rule: Option < Vec < String >> ,

    /// Limit read rate (bytes per second) from a device
     # [arg(long, value_name = "list")]
    device_read_bps: Option < Vec < String >> ,

    /// Limit read rate (IO per second) from a device
     # [arg(long, value_name = "list")]
    device_read_iops: Option < Vec < String >> ,

    /// Limit write rate (bytes per second) to a device
     # [arg(long, value_name = "list")]
    device_write_bps: Option < Vec < String >> ,

    /// Limit write rate (IO per second) to a device
     # [arg(long, value_name = "list")]
    device_write_iops: Option < Vec < String >> ,

    /// Skip image verification (default true)
     # [arg(long)]
    disable_content_trust: bool,

    /// Set custom DNS servers
     # [arg(long, value_name = "list")]
    dns: Option < Vec < String >> ,

    /// Set DNS options
     # [arg(long, value_name = "list")]
    dns_option: Option < Vec < String >> ,

    /// Set custom DNS search domains
     # [arg(long, value_name = "list")]
    dns_search: Option < Vec < String >> ,

    /// Container NIS domain name
     # [arg(long)]
    domainname: Option < String > ,

    /// Overwrite the default ENTRYPOINT of the image
     # [arg(long)]
    entrypoint: Option < String > ,

    /// Set environment variables
     # [arg(short = 'e', long, value_name = "list")]
    env: Option < Vec < String >> ,

    /// Read in a file of environment variables
     # [arg(long, value_name = "list")]
    env_file: Option < Vec < String >> ,

    /// Expose a port or a range of ports
     # [arg(long, value_name = "list")]
    expose: Option < Vec < String >> ,

    /// GPU devices to add to the container ('all' to pass all GPUs)
     # [arg(long)]
    gpus: Option < String > ,

    /// Add additional groups to join
     # [arg(long, value_name = "list")]
    group_add: Option < Vec < String >> ,

    /// Command to run to check health
     # [arg(long)]
    health_cmd: Option < String > ,

    /// Time between running the check (ms|s|m|h)
     # [arg(long)]
    health_interval: Option < String > ,

    /// Consecutive failures needed to report unhealthy
     # [arg(long)]
    health_retries: Option < u32 > ,

    /// Start period for the container to initialize
     # [arg(long)]
    health_start_period: Option < String > ,

    /// Maximum time to allow one check to run (ms|s|m|h)
     # [arg(long)]
    health_timeout: Option < String > ,

    /// Container host name
     # [arg(short = 'H', long)]
    hostname: Option < String > ,

    /// Run an init inside the container
     # [arg(long)]
    init: bool,

    /// Keep STDIN open even if not attached
     # [arg(short = 'i', long)]
    interactive: bool,

    /// IPv4 address
     # [arg(long)]
    ip: Option < String > ,

    /// IPv6 address
     # [arg(long)]
    ip6: Option < String > ,

    /// IPC mode to use
     # [arg(long)]
    ipc: Option < String > ,

    /// Container isolation technology
     # [arg(long)]
    isolation: Option < String > ,

    /// Kernel memory limit
     # [arg(long)]
    kernel_memory: Option < String > ,

    /// Set meta data on a container
     # [arg(short = 'l', long, value_name = "list")]
    label: Option < Vec < String >> ,

    /// Read in a line delimited file of labels
     # [arg(long)]
    label_file: Option < Vec < String >> ,

    /// Add link to another container
     # [arg(long, value_name = "list")]
    link: Option < Vec < String >> ,

    /// Container IPv4/IPv6 link-local addresses
     # [arg(long, value_name = "list")]
    link_local_ip: Option < Vec < String >> ,

    /// Logging driver for the container
     # [arg(long)]
    log_driver: Option < String > ,

    /// Log driver options
     # [arg(long, value_name = "list")]
    log_opt: Option < Vec < String >> ,

    /// Container MAC address
     # [arg(long)]
    mac_address: Option < String > ,

    /// Memory limit
     # [arg(short = 'm', long)]
    memory: Option < String > ,

    /// Memory soft limit
     # [arg(long)]
    memory_reservation: Option < String > ,

    /// Swap limit equal to memory plus swap: '-1' to enable unlimited swap
     # [arg(long)]
    memory_swap: Option < String > ,

    /// Tune container memory swappiness (0 to 100)
     # [arg(long)]
    memory_swappiness: Option < i32 > ,

    /// Attach a filesystem mount to the container
     # [arg(long, value_name = "mount")]
    mount: Option < String > ,

    /// Assign a name to the container
     # [arg(long)]
    name: Option < String > ,

    /// Connect a container to a network
     # [arg(long)]
    network: Option < String > ,

    /// Add network-scoped alias for the container
     # [arg(long, value_name = "list")]
    network_alias: Option < Vec < String >> ,

    /// Disable any container-specified HEALTHCHECK
     # [arg(long)]
    no_healthcheck: bool,

    /// Disable OOM Killer
     # [arg(long)]
    oom_kill_disable: bool,

    /// Tune host's OOM preferences (-1000 to 1000)
     # [arg(long)]
    oom_score_adj: Option < i32 > ,

    /// PID namespace to use
     # [arg(long)]
    pid: Option < String > ,

    /// Tune container pids limit (-1 for unlimited)
     # [arg(long)]
    pids_limit: Option < i32 > ,

    /// Set platform if server is multi-platform capable
     # [arg(long)]
    platform: Option < String > ,

    /// Give extended privileges to this container
     # [arg(long)]
    privileged: bool,

    /// Publish a container's port(s) to the host
     # [arg(short = 'p', long, value_name = "list")]
    publish: Option < Vec < String >> ,

    /// Publish all exposed ports to random ports
     # [arg(short = 'P', long)]
    publish_all: bool,

    /// Pull image before running
     # [arg(long)]
    pull: Option < String > ,

    /// Mount the container's root filesystem as read only
     # [arg(long)]
    read_only: bool,

    /// Restart policy to apply when a container exits
     # [arg(long)]
    restart: Option < String > ,

    /// Automatically remove the container when it exits
     # [arg(long)]
    rm: bool,

    /// Security options
     # [arg(long, value_name = "list")]
    security_opt: Option < Vec < String >> ,

    /// Set the container's storage driver options per-mount
     # [arg(long, value_name = "list")]
    storage_opt: Option < Vec < String >> ,

    /// Stop signal to use
     # [arg(long)]
    stop_signal: Option < String > ,

    /// Timeout (in seconds) to stop the container
     # [arg(long)]
    stop_timeout: Option < i32 > ,

    /// Kernel parameters to set in the container
     # [arg(long, value_name = "list")]
    sysctl: Option < Vec < String >> ,

    ///  Bind mount a volume
     # [arg(short, long, value_name = "list")]
    volume: Option < Vec < String >> ,
    // volume: Option<String>,

    /// Working directory inside the container
     # [arg(short, long)]
    workdir: Option < String >
}

 # [derive(Serialize, Deserialize)]
struct ResponseData {
     # [serde(rename = "resultCode")]
    result_code: i32,
    output: String,
    error: String,
}

pub async fn exec(container: String, code: String)->Result < (), Box < dyn Error >> {
    // pub async fn exec(container: String, code: String, file_path: Option<String>) -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    // let url = "https://cpprunner.aiursoft.cn/runner/run?lang=rust";
    let url = format!("https://cpprunner.aiursoft.cn/runner/run?lang={}", container);
    // let code = r#"fn main() { println!("hello world"); }"#;
    // let code = command.clone();

    let response = client
        .post(url)
        .header("Content-Type", "text/plain")
        .body(code)
        .send()
        .await ? ;

    let body = response.text().await ? ;
    // println!("Response: {}", body);
    // 反序列化 JSON 响应为 ResponseData 结构体
    let response_data : ResponseData = serde_json::from_str( & body) ? ;

    // 打印反序列化后的数据
    // println!("{:#}", response_data);
    println!("{}", response_data.result_code);
    println!("{}", response_data.output);
    println!("{}", response_data.error);

    Ok(())
}

async fn ls() {
    println!("C (gcc 9.5.0)");
    println!("C++ (GNU G++, stdc++20)");
    println!("CUDA 11.6 (on Ubuntu 20.04)");
    println!("C# (.NET 7.0)");
    println!("Go (Golang 1.21.5)");
    println!("Rust (1.74.1)");
    println!("Javascript (Node.js v21)");
    println!("TypeScript (4.9.3, node 16.8.1)");
    println!("Python (CPython 3.11)");
    println!("Python with PyTorch (Pytorch 2.3.0; cuda 11.8; cudnn 8)");
    println!("Bash (on Ubuntu 24.04)");
    println!("PowerShell Core (Ubuntu 22.04)");
    println!("Swift (5.8.1)");
    println!("Java (OpenJDK 23)");
    println!("Ruby (3.2.2)");
    println!("PHP (8.3.0)");
    println!("Perl (5.39.5)");
    println!("Lua (5.4)");
    println!("Haskell (GHC 9.8.1)");
    println!("Lisp (rigetti/lisp)");
}
