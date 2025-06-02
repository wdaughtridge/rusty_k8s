use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .include_file("k8s.rs")
        .message_attribute(
            ".",
            "#[derive(k8s_macro::K8sResource)] #[derive(serde::Serialize, serde::Deserialize)]",
        )
        .compile_protos(&["proto/k8s.io/api/apps/v1/generated.proto"], &["proto"])?;

    Ok(())
}
