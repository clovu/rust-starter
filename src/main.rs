mod error;

use error::Result;
use file_macro::FileOps;

fn main() -> Result<()> {
    // load env profile
    dotenv::dotenv()?;

    let files = FileUtils::list_file_path(".")?;

    println!("Hello, world! {:?}", files);

    Ok(())
}

#[derive(FileOps)]
struct FileUtils;
