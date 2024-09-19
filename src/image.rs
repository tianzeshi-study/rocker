use futures::StreamExt;
use shiplift::{BuildOptions, Docker};

pub async fn build(path: &str) {
    let docker = Docker::new();
    // let path = env::args().nth(1).expect("You need to specify a path");
let tag ="latest"; 

    // let options = BuildOptions::builder(path).tag("shiplift_test").build();
    let options = BuildOptions::builder(path).tag(tag).build();

    let mut stream = docker.images().build(&options);
    while let Some(build_result) = stream.next().await {
        match build_result {
            Ok(output) => println!("{:?}", output),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

pub async fn rmi(image: [&str]) {
    let docker = Docker::new();
    // let img = env::args().nth(1).expect("You need to specify an image name");

    let img = image[0];
    
    match docker.images().get(&img).delete().await {
        Ok(statuses) => {
            for status in statuses {
                println!("{:?}", status);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
