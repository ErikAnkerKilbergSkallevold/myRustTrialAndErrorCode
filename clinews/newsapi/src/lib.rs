use serde::Deserialize;
use url::Url;

const BASE_URL: &str = "https://newsapi.org/v2/";

///Error handler for the NewsAPI. 
#[derive(thiserror::Error, Debug)]
pub enum NewsApiError{
    #[error("Failed fetching articles")]
    RequestFailed(#[from] ureq::Error),
    #[error("Failed converting response to string")]
    FailedToConvertResponseToString(#[from] std::io::Error),
    #[error("Failed parsing articles")]
    FailedToParseArticles(#[from] serde_json::Error),
    #[error("URL parsing failed")]
    UrlParsingFailed(#[from] url::ParseError),
    #[error("Request failed: {0}")]
    BadRequest(&'static str),
}

///Struct containing all the articles in a vector returned by the NewsAPI.
#[derive(Deserialize, Debug)]
pub struct NewsAPIResponse{
    status: String,
    pub articles: Vec<Article>,
    code: Option<String>
}

impl NewsAPIResponse{
    ///Returns a vector of articles.
    pub fn articles(&self) -> &Vec<Article>{
        &self.articles
    }
}

///Struct containing the article returned by the NewsAPI.
#[derive(Deserialize, Debug)]
pub struct Article{
    title: String,
    url: String
}
///Returns the title and URL of the article.
impl Article{
    pub fn title(&self) -> &str{
        &self.title
    }

    pub fn url(&self) -> &str{
        &self.url
    }
}

//pub fn get_articles(url:&str) -> Result<Articles, NewsApiError> {
//    let response = ureq::get(url).call().map_err(|e| NewsApiError::RequestFailed(e))
//    ?.into_string().map_err(|e| NewsApiError::FailedToConvertResponseToString(e))?;
//
//    let articles: Articles = serde_json::from_str(&response).map_err(|e| NewsApiError::FailedToParseArticles(e))?;
//
//    Ok(articles)
//}

///Enum containing the endpoint of the NewsAPI.
pub enum Endpoint {
    TopHeadlines
}

///Implements toString for the Endpoint enum.
impl ToString for Endpoint {
    fn to_string(&self) -> String {
        match self {
            Self::TopHeadlines => "top-headlines".to_string()
        }
    }
}

///Enum for the chosen country.
pub enum Country {
    Us
}

///Implements toString for the Country enum.
impl ToString for Country {
    fn to_string(&self) -> String {
        match self {
            Self::Us => "Us".to_string()
        }
    }
}

///Struct containing the NewsAPI.
pub struct  NewsAPI {
    api_key: String,
    endpoint: Endpoint,
    country: Country
}

///Implements the NewsAPI struct.
impl NewsAPI{
    pub fn new(api_key: &str) -> NewsAPI{
        NewsAPI{
            api_key: api_key.to_string(),
            endpoint: Endpoint::TopHeadlines,
            country: Country::Us
        }
    }
    ///Sets the endpoint.
    pub fn endpoint(&mut self, endpoint: Endpoint) -> &mut NewsAPI{
        self.endpoint = endpoint;
        self
    }
    ///Sets the country.
    pub fn country(&mut self, country: Country) -> &mut NewsAPI{
        self.country = country;
        self
    }
    ///Returns the URL of the NewsAPI.
    fn prepare_url(&self) -> Result<String, NewsApiError> {
        let mut url = Url::parse(BASE_URL)?;

        url.path_segments_mut().unwrap().push(&self.endpoint.to_string());
        
        let country = format!("country={}", self.country.to_string());
        url.set_query(Some(&country));

        Ok(url.to_string())
    }
    ///Returns the response of the NewsAPI.
    pub fn fetch(&self) -> Result<NewsAPIResponse, NewsApiError> {
        let url = self.prepare_url()?;
        let req = ureq::get(&url).set("Authorization", &self.api_key);
        let response: NewsAPIResponse = req.call()?.into_json()?;
        match response.status.as_str() {
            "ok" => return Ok(response),
            _ => return Err(map_response_err(response.code)),
        }
    }
}

///Maps the response code to an error.
fn map_response_err(code: Option<String>) -> NewsApiError{
    if let Some(code) = code {
        match code.as_str() {
            "apiKeyDisabled" => NewsApiError::BadRequest("Your API key has been disabled"),
            _ => NewsApiError::BadRequest("Unknown error"),
        }
    } else {
        NewsApiError::BadRequest("Unknown error")
    }
}
