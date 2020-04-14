fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::fs::metadata("src/generated").is_ok() {
    std::fs::remove_dir_all("src/generated")?;
    }
    std::fs::create_dir("src/generated")?;
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir("src/generated")
        .compile(
        &[
            "./proto/pokedex.proto"],
        &["./proto"],
        )?;
    Ok(())
}