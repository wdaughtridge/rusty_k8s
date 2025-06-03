use k8s::k8s::io::{api::core::v1::Pod, apimachinery::pkg::apis::meta::v1::ObjectMeta};

mod k8s {
    tonic::include_proto!("k8s");
}

fn main() {
    let mut my_pod = Pod::new();

    let mut meta = ObjectMeta::new();
    meta.set_name("my-pod".to_string());
    meta.set_namespace("devops".to_string());

    my_pod.set_metadata(meta);
}
