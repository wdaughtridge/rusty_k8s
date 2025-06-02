mod k8s {
    tonic::include_proto!("k8s");
}

use crate::k8s::k8s::io::api::core::v1 as core_v1;
use crate::k8s::k8s::io::apimachinery::pkg::apis::meta::v1 as meta_v1;

fn main() {
    let mut my_pod = core_v1::Pod {
        ..Default::default()
    };

    my_pod.set_metadata(Some(meta_v1::ObjectMeta {
        name: Some("test-pod".to_string()),
        ..Default::default()
    }));
}
