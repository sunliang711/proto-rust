pub fn compile(source_file: &str, includes: &[&str]) -> anyhow::Result<()> {
    prost_build::compile_protos(&[source_file], includes)?;

    Ok(())
}
