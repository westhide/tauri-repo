use tonic_build::Config;

fn main() -> std::io::Result<()> {
    let header = &["./proto"];
    let protos = &["./proto/internal.proto"];

    let mut config = Config::new();
    config.prost_path("crate::prost");

    tonic_build::configure().compile_protos_with_config(config, protos, header)
}
