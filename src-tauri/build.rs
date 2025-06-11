fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().compile_protos(
        &["../proto/internal.proto", "../proto/helloworld.proto"],
        &["../proto"],
    )?;
    tauri_build::build();
    Ok(())
}
