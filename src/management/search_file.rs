// TODO: Add documentation to the functions
// TODO: Improve function names
// TODO: Implement the builder pattern to be able to chain the searching functions and make the code more readable (Yep it'll lead us to rewrite this file from scratch)

use std::fmt::Display;

use crate::client::{ImageKit, FILES_ENDPOINT};
use crate::error::{Error, Result};
use crate::upload::types::Response;
use async_trait::async_trait;
use reqwest::{Request, Url};

/// An extension trait for joining slices of any type that implements Display
/// The intention is to use it in the RangeSearch::between and RangeSearch::not_between functions
/// Example:
/// let formats = FormatOpts::between(&[FormatOpts::Jpg, FormatOpts::Png, FormatOpts::Webp]);
/// let formats = FormatOpts::not_between(&[FormatOpts::Jpg, FormatOpts::Png, FormatOpts::Webp]);

trait JoinExt<T: Display> {
    fn join(&self, separator: &str) -> String;
}

impl JoinExt<FormatOpts> for [FormatOpts] {
    fn join(&self, separator: &str) -> String {
        self.iter()
            .map(|item| format!("'{}'", item))
            .collect::<Vec<String>>()
            .join(separator)
    }
}
/// RequestCriteria for searching the Media library
pub struct RequestCriteria(Request);
impl RequestCriteria {
    pub fn new(query_params: &[(String, String)]) -> Self {
        let url = Url::parse_with_params(FILES_ENDPOINT, query_params)
            .expect("Could not parse query params");
        let req = Request::new(reqwest::Method::GET, url);
        Self(req)
    }
}
/// A trait for performing comparion searches on the Media Library. It returns SearchRequestCriteria
trait ComparisonSearch<T> {
    fn greater(self, right: &T) -> RequestCriteria;
    fn greater_eq(self, right: &T) -> RequestCriteria;
    fn less(self, right: &T) -> RequestCriteria;
    fn less_eq(self, right: &T) -> RequestCriteria;
}
/// A trait for performing in-range searches on the Media Library. It returns SearchRequestCriteria
trait RangeSearch<T> {
    fn between(range: &[T]) -> RequestCriteria;
    fn not_between(range: &[T]) -> RequestCriteria;
}
/// A trait for performing searches based-on partial equality (prefixes) or full equality on the Media Library. It returns SearchRequestCriteria
trait CommonSearch<T> {
    fn eq(right: &T) -> RequestCriteria;
}

pub enum FormatOpts {
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
impl CommonSearch<FormatOpts> for FormatOpts {
    fn eq(right: &FormatOpts) -> RequestCriteria {
        let format = format!("format=\"{}\"", right);
        RequestCriteria::new(&[("searchQuery".to_string(), format)])
    }
}
impl RangeSearch<FormatOpts> for FormatOpts {
    fn between(range: &[FormatOpts]) -> RequestCriteria {
        let formats = format!("format IN [{}]", range.join(","));
        RequestCriteria::new(&[("searchQuery".to_string(), formats)])
    }
    fn not_between(range: &[FormatOpts]) -> RequestCriteria {
        let formats = format!("format NOT IN [{}]", range.join(","));
        RequestCriteria::new(&[("searchQuery".to_string(), formats)])        
    }
}

// implementing Display for FormatOpts so that we can use it in the error message and we avoid injecting a match statement in the eq function
impl Display for FormatOpts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FormatOpts::Jpg => "jpg",
                FormatOpts::Webp => "webp",
                FormatOpts::Png => "png",
                FormatOpts::Gif => "gif",
                FormatOpts::Svg => "svg",
                FormatOpts::Avif => "avif",
                FormatOpts::Pdf => "pdf",
                FormatOpts::Js => "js",
                FormatOpts::Woff2 => "woff2",
                FormatOpts::Woff => "woff",
                FormatOpts::Ttf => "ttf",
                FormatOpts::Otf => "otf",
                FormatOpts::Eot => "eot",
                FormatOpts::Css => "css",
                FormatOpts::Txt => "txt",
                FormatOpts::Mp4 => "mp4",
                FormatOpts::Webm => "webm",
                FormatOpts::Mov => "mov",
                FormatOpts::Swf => "swf",
                FormatOpts::Ts => "ts",
                FormatOpts::M3u8 => "m3u8",
                FormatOpts::Ico => "ico",

            }
        )
    }
}
pub struct Filename(String);
impl Filename {
    pub fn new(name: &str) -> Self {
        Self(name.to_string())
    }
}

impl CommonSearch<Filename> for Filename {
    fn eq(right: &Filename) -> RequestCriteria {
        let format = format!("name=\"{}\"", right.0);
        RequestCriteria::new(&[("searchQuery".to_string(), format)])
    }
}

#[async_trait]
pub trait Search {
    /// Search files by the given FormatOpts, in case no file were found by the given searching-criteria it will return a search error
    async fn search_by_format(&self, criteria: FormatOpts) -> Result<Vec<Response>>;
    ///Search files by a given collection of FormatOpts, in case no file were found by the given searching-criteria it will return a search error
    async fn search_by_formats(&self, criteria: &[FormatOpts]) -> Result<Vec<Response>>;
    async fn search_by_formats_not_in_range(&self, criteria: &[FormatOpts]) -> Result<Vec<Response>>;
    /// Search files by the given Filename (including the file extension), in case no file were found by the given searching-criteria it will return an empty vector
    async fn search_by_filename(&self, criteria: Filename) -> Result<Response>;
    /// Send a request to the imagekit API to search files by the given RequestCriteria. In case no file were found by the given searching-criteria it will return a search error
    async fn send(&self, params: RequestCriteria) -> Result<Vec<Response>>;
}

#[async_trait]
impl Search for ImageKit {
    async fn search_by_format(&self, criteria: FormatOpts) -> Result<Vec<Response>> {
        let eq = FormatOpts::eq(&criteria);
        let response = self.send(eq).await?;
        if !response.is_empty() {
            return Ok(response);
        } else {
            return Err(Error::SearchError(format!(
                "No files were found by the given format: {}",
                criteria
            )));
        }
    }
    async fn search_by_formats(&self, criteria: &[FormatOpts]) -> Result<Vec<Response>> {
        let between = FormatOpts::between(criteria);
        let response = self.send(between).await?;
        if !response.is_empty() {
            return Ok(response);
        } else {
            return Err(Error::SearchError(format!(
                "No files were found by the given formats: {}",
                criteria.join(",")
            )));
        }
    }
    async fn search_by_formats_not_in_range(&self, criteria: &[FormatOpts]) -> Result<Vec<Response>> {
        let not_between = FormatOpts::not_between(criteria);
        let response = self.send(not_between).await?;
        if !response.is_empty() {
            return Ok(response);
        } else {
            return Err(Error::SearchError(format!(
                "No files were found by the given formats: {}",
                criteria.join(",")
            )));
        }
    }
    async fn search_by_filename(&self, criteria: Filename) -> Result<Response> {
        let eq = Filename::eq(&criteria);
        let response = self.send(eq).await?;
        let response = response.first();
        if let Some(res) = response {
            return Ok(res.clone());
        } else {
            return Err(Error::SearchError(format!(
                "No files were found by the given name: {}",
                criteria.0
            )));
        }
    }
    async fn send(&self, params: RequestCriteria) -> Result<Vec<Response>> {
        let req = params.0;
        let response = self.client.execute(req).await?;

        if response.status().is_success() {
            let result: Vec<Response> = serde_json::from_str(&response.text().await?)?;
            return Ok(result);
        }

        let error = Error::from_error_code(response.status(), &response.text().await?);
        return Err(error);
    }
}
