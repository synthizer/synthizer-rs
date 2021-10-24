//! Show how to set up the asset_lru support.
//!
//! This example takes two arguments: an absolute path to a directory, and a
//! file within that directory.  It then sets up a `AssetCache` and opens via
//! that.
use anyhow::Result;

use asset_lru::AssetCache;
use synthizer as syz;

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 3 {
        panic!("Usage: example <root> <file>");
    }

    let root_path = &args[1];
    let key = &args[2];

    let _guard = syz::initialize()?;

    // Set up an asset cache with the filesystem vfs.
    let cache_config = asset_lru::AssetCacheConfig {
        max_bytes_cost: 10000000,
        max_decoded_cost: 10000000,
        max_single_object_bytes_cost: 1000000,
        max_single_object_decoded_cost: 1000000,
    };
    let vfs = asset_lru::FilesystemVfs::new(std::path::Path::new(root_path.as_str()))?;
    let cache = AssetCache::new(vfs, syz::BufferAssetLruDecoder::new(), cache_config);

    let buf = cache.get(&key)?;
    let ctx = syz::Context::new()?;
    let gen = syz::BufferGenerator::new(&ctx)?;
    let src = syz::DirectSource::new(&ctx)?;
    gen.buffer().set(&*buf)?;
    src.add_generator(&gen)?;

    println!("Press enter to exit...");
    std::io::stdin().read_line(&mut String::new()).unwrap();

    Ok(())
}
