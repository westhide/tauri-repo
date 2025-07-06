fn main() -> std::io::Result<()> {
    let header = &["./proto"];
    let protos = &["./proto/internal.proto"];
    tonic_build::configure().compile_protos(protos, header)
}
