use imagekit::{
    self,
    search_file::{Filename, FormatOpts, Search},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = imagekit::ImageKit::from_env()?;
    let search_png = client.search_by_format(FormatOpts::Png).await?;
    let search_by_name = client
        .search_by_filename(Filename::new("linus torvalds lv 14.png"))
        .await?;
    let search_png_jpg = client
        .search_by_formats(&[
            FormatOpts::Png,
            FormatOpts::Jpg,
        ])
        .await?;
    let search_by_formats_not_png = client.search_by_formats_not_in_range(&[FormatOpts::Png]).await?;
    println!("Search all the files by PNG --- {:#?}", search_png);
    println!("Search the file by the given name --- {:#?}", search_by_name);
    println!("Search all the files within the given formats --- {:#?}", search_png_jpg);
    println!("Search all the files not within the given formats --- {:#?}", search_by_formats_not_png);
    Ok(())
}
