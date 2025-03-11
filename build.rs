use std::io::Result;
fn main() -> Result<()> {
    prost_build::Config::new()
        // if you need Serde features on all these types
        .message_attribute(".", "#[derive(serde::Serialize)]")
        .compile_protos(&["src/gtfs-realtime.proto"], &["src/"])?;
    Ok(())
}
