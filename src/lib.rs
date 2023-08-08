pub mod client;
pub mod delete;
pub mod error;
pub mod management;
pub mod types;
pub mod upload;

pub use client::ImageKit;
pub use delete::Delete;
pub use management::file_details;
pub use management::search_file;
pub use upload::Upload;

#[cfg(test)]
mod tests {
    use tokio::fs::File;

    use super::delete::Delete;
    use super::file_details::Details;
    use super::search_file::{FormatOptions, Search};
    use super::upload::types::FileType;
    use super::upload::{Options, Upload, UploadFile};
    use super::ImageKit;

    #[tokio::test]
    async fn uploads_then_deletes_file() {
        let imagekit = ImageKit::from_env().unwrap();
        let file = File::open("assets/ferris.jpeg").await.unwrap();
        let upload_file = UploadFile::from(file);
        let opts = Options::new(upload_file, "ferris");
        let upload_result = imagekit.upload(opts).await.unwrap();

        assert_eq!(upload_result.file_type, FileType::Image);
        assert_eq!(upload_result.height.unwrap(), 640);
        assert_eq!(upload_result.width.unwrap(), 640);

        let detail_result = imagekit.get_file_details(&upload_result.file_id).await;
        assert!(detail_result.is_ok());

        let delete_result = imagekit.delete(upload_result.file_id).await;

        assert!(delete_result.is_ok());
    }
    #[tokio::test]
    async fn search_file() {
        let imagekit = ImageKit::from_env().unwrap();
        // To make a search based on formats
        let search_result = imagekit.search_by_format(FormatOptions::Jpg).await;
        assert!(search_result.is_ok());

    }
    #[tokio::test]
    async fn uploads_and_retrieve_information() {
        let imagekit = ImageKit::from_env().unwrap();
        let file = File::open("assets/ferris.jpeg").await.unwrap();
        let upload_file = UploadFile::from(file);
        let opts = Options::new(upload_file, "ferris");
        let upload_result = imagekit.upload(opts).await.unwrap();
        let detail_result = imagekit.get_file_details(upload_result.file_id).await;
        assert!(detail_result.is_ok());
    }
}
