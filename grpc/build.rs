fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_path = format!("{}/proto/videoprocessing.proto", env!("CARGO_MANIFEST_DIR"));
    tonic_build::compile_protos(proto_path)?;
    Ok(())
}
