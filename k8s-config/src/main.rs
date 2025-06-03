mod proto {
    tonic::include_proto!("k8s");
}

use proto::k8s::io::{
    api::core::v1::{Container, Pod, PodSpec},
    apimachinery::pkg::apis::meta::v1::ObjectMeta,
};

fn main() {
    let mut my_pod = Pod::new();

    let mut metadata = ObjectMeta::new();
    metadata.set_name("my-pod".to_string());
    metadata.set_namespace("devops".to_string());
    my_pod.set_metadata(metadata);

    let mut my_container = Container::new();
    my_container.set_name("busy-pod".to_string());
    my_container.set_image("busybox:latest".to_string());
    my_container.set_image_pull_policy("Always".to_string());
    my_container.set_command(
        [
            "/bin/sh".to_string(),
            "-c".to_string(),
            "'while true; do echo hello; sleep 120; done'".to_string(),
        ]
        .to_vec(),
    );

    let mut my_spec = PodSpec::new();
    my_spec.set_containers([my_container].to_vec());

    my_pod.set_spec(my_spec);

    println!("{}", serde_json::to_string_pretty(&my_pod).unwrap());
}
