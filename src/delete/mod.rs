use anyhow::Result;
use async_trait::async_trait;
use reqwest::{StatusCode, Url};

use crate::client::FILES_ENDPOINT;
use crate::{ErrorResponse, ImageKit, error::Error};

#[async_trait]
pub trait Delete {
    /// Deletes the file with the provided File ID
    async fn delete<T: ToString + Send>(&self, file_id: T) -> Result<(), Error>;
}

#[async_trait]
impl Delete for ImageKit {
    async fn delete<T: ToString + Send>(&self, file_id: T) -> Result<(), Error> {
        let url_string = format!("{}/{}", FILES_ENDPOINT, file_id.to_string());
        let endpoint_url = Url::parse(&url_string).unwrap();
        let response = self.client.delete(endpoint_url).send().await.unwrap();

        if matches!(response.status(), StatusCode::NO_CONTENT) {
            return Ok(());
        }
        let res_status_code = response.status().as_u16();
        let result = response.json::<ErrorResponse>().await.unwrap();

        Err(Error::ParsingError(result.message, res_status_code))
    }
}
