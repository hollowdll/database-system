// Build script to compile .proto files into Rust files during compilation.

use std::io::Result;

fn main() -> Result<()> {
    prost_build::compile_protos(&["src/proto/database.proto"], &["src/proto"])?;
    Ok(())
}
