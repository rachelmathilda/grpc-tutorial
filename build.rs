fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir(std::env::var("OUT_DIR")?)
        .compile_protos(&["proto/services.proto"], &["proto"])?;
    Ok(())
}