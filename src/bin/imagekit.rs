use imagekit::{
    self,
    search_file::{FormatOptions, Search},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = imagekit::ImageKit::from_env()?;
    let a = client.search_by_format(FormatOptions::Jpg).await?;
    println!("{:#?}", a);
    Ok(())
}
