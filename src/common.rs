//! Common module with common used structures across different
//! commands.
use std::env;
use std::path::PathBuf;
use structopt::StructOpt;
use wasmer::VERSION;

#[derive(Debug, StructOpt, Clone)]
/// The WebAssembly features that can be passed through the
/// Command Line args.
pub struct WasmFeatures {
    /// Enable support for the SIMD proposal.
    #[structopt(long = "enable-simd")]
    pub simd: bool,

    /// Enable support for the threads proposal.
    #[structopt(long = "enable-threads")]
    pub threads: bool,

    /// Enable support for the reference types proposal.
    #[structopt(long = "enable-reference-types")]
    pub reference_types: bool,

    /// Enable support for the multi value proposal.
    #[structopt(long = "enable-multi-value")]
    pub multi_value: bool,

    /// Enable support for the bulk memory proposal.
    #[structopt(long = "enable-bulk-memory")]
    pub bulk_memory: bool,

    /// Enable support for all pre-standard proposals.
    #[structopt(long = "enable-all")]
    pub all: bool,
}

#[derive(Debug, Clone, StructOpt)]
/// The compiler options
pub struct Compiler {
    /// Use Singlepass compiler
    #[structopt(long, conflicts_with_all = &["cranelift", "llvm"])]
    singlepass: bool,

    /// Use Cranelift compiler
    #[structopt(long, conflicts_with_all = &["singlepass", "llvm"])]
    cranelift: bool,

    /// Use LLVM compiler
    #[structopt(long, conflicts_with_all = &["singlepass", "cranelifft"])]
    llvm: bool,

    #[structopt(flatten)]
    features: WasmFeatures,
}

/// Get the cache dir
pub fn get_cache_dir() -> PathBuf {
    match env::var("WASMER_CACHE_DIR") {
        Ok(dir) => {
            let mut path = PathBuf::from(dir);
            path.push(VERSION);
            path
        }
        Err(_) => {
            // We use a temporal directory for saving cache files
            let mut temp_dir = env::temp_dir();
            temp_dir.push("wasmer");
            temp_dir.push(VERSION);
            temp_dir
        }
    }
}
