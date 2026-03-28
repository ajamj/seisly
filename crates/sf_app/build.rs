fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../sf_core/proto/analysis.proto")?;
    Ok(())
}
