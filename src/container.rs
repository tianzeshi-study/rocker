use shiplift::{Docker, ContainerListOptions};


pub async fn ps() {
    // env_logger::init();
    let docker = Docker::new();
    match docker.containers().list(&ContainerListOptions::default()).await {
        Ok(containers) => {
            for c in containers {
                println!("container -> {:#?}", c)
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}