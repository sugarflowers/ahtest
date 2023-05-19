use std::error::Error;
use anyhow::Context as _;

fn my_func() -> Result<(), Box<dyn Error>> {
    std::fs::File::open("test.txt").with_context(|| "ファイル読込みのエラー")?;
    Ok(())
}

/*
fn main() {
    match my_func() {
        Ok(_) => println!("ok!"),
        Err(e) => panic!("\n\n\n{:?}\n\n\n----", e.to_string()),
    }
}
*/
