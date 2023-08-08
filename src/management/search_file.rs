use crate::client::{ImageKit, FILES_ENDPOINT};
use crate::error::{Error, Result};
use crate::upload::types::Response;
use async_trait::async_trait;
use reqwest::{Request, StatusCode, Url};

/// Parameters for searching the Media library
pub struct Parameters(Request);
impl Parameters {
    pub fn new(query_params: &[(String, String)]) -> Self {
        let url = Url::parse_with_params(FILES_ENDPOINT, query_params)
            .expect("Could not parse query params");
        let req = Request::new(reqwest::Method::GET, url);
        Self(req)
    }
}
/// A trait for performing comparion searches on the Media Library. It returns SearchParameters
trait ComparisonSearch<T> {
    fn greater(self, right: &T) -> Parameters;
    fn greater_eq(self, right: &T) -> Parameters;
    fn less(self, right: &T) -> Parameters;
    fn less_eq(self, right: &T) -> Parameters;
}
/// A trait for performing in-range searches on the Media Library. It returns SearchParameters
trait RangeSearch<T> {
    fn between(self, left: &T, right: &T) -> Parameters;
    fn not_between(self, left: &T, right: &T) -> Parameters;
}
/// A trait for performing searches based-on partial equality (prefixes) or full equality on the Media Library. It returns SearchParameters
trait CommonSearch<T> {
    fn eq(right: &T) -> Parameters;
}

pub enum FormatOptions {
    Jpg,
    Webp,
    Png,
    Gif,
    Svg,
    Avif,
    Pdf,
    Js,
    Woff2,
    Woff,
    Ttf,
    Otf,
    Eot,
    Css,
    Txt,
    Mp4,
    Webm,
    Mov,
    Swf,
    Ts,
    M3u8,
    Ico,
}
pub struct Filename(String);
impl Filename {
    pub fn new(name: &str) -> Self {
        Self(name.to_string())
    }
}
impl CommonSearch<FormatOptions> for FormatOptions {
    fn eq(right: &FormatOptions) -> Parameters {
        let format = format!(
            "format=\"{}\"",
            match right {
                FormatOptions::Jpg => "jpg",
                FormatOptions::Webp => "webp",
                FormatOptions::Png => "png",
                FormatOptions::Gif => "gif",
                FormatOptions::Svg => "svg",
                FormatOptions::Avif => "avif",
                FormatOptions::Pdf => "pdf",
                FormatOptions::Js => "js",
                FormatOptions::Woff2 => "woff2",
                FormatOptions::Woff => "woff",
                FormatOptions::Ttf => "ttf",
                FormatOptions::Otf => "otf",
                FormatOptions::Eot => "eot",
                FormatOptions::Css => "css",
                FormatOptions::Txt => "txt",
                FormatOptions::Mp4 => "mp4",
                FormatOptions::Webm => "webm",
                FormatOptions::Mov => "mov",
                FormatOptions::Swf => "swf",
                FormatOptions::Ts => "ts",
                FormatOptions::M3u8 => "m3u8",
                FormatOptions::Ico => "ico",
            }
        );
        Parameters::new(&[("searchQuery".to_string(), format)])
    }
}
#[async_trait]
pub trait Search {
    async fn search_by_format(&self, criteria: FormatOptions) -> Result<Vec<Response>>;
    async fn search_by_filename(&self, criteria: Filename) -> Result<()>;
    async fn send(&self, parameters: Parameters) -> Result<Vec<Response>>;
}

#[async_trait]
impl Search for ImageKit {
    async fn search_by_format(&self, criteria: FormatOptions) -> Result<Vec<Response>> {
        let eq = FormatOptions::eq(&criteria);
        let response = self.send(eq).await?;
        Ok(response)
    }
    async fn search_by_filename(&self, criteria: Filename) -> Result<()> {
        Ok(())
    }
    async fn send(&self, parameters: Parameters) -> Result<Vec<Response>> {
        let req = parameters.0;
        let response = self.client.execute(req).await?;
        
        if matches!(response.status(), StatusCode::OK) {
            let result: Vec<Response> = serde_json::from_str(&response.text().await?)?;

            return Ok(result);
        }

        let error = Error::from_error_code(response.status(), &response.text().await?);
        return Err(error);
    }
}
