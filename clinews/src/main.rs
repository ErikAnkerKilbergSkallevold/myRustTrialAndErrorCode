mod theme;

use std::error::Error;
use dotenv::dotenv;
use newsapi::{NewsAPI, Country, Endpoint, Article};

///Renders the title and URL of the article in the terminal with custom theme.
fn render_articles(articles: &Vec<Article>) {
    let theme = theme::default();
    theme.print_text("# Top headlines \n\n");
    for a in articles {
        theme.print_text(&format!("'{}'", a.title()));
        theme.print_text(&format!("> *{}*", a.url()));
        theme.print_text("---");
    }
}


fn main() -> Result<(), Box<dyn Error>>{
    dotenv().ok();

    let api_key = std::env::var("API_KEY")?;

    let mut newsapi = NewsAPI::new(&api_key);
    newsapi.endpoint(Endpoint::TopHeadlines).country(Country::Us);
   
    let newsapi_response = newsapi.fetch()?;

    render_articles(&newsapi_response.articles());

    Ok(())
}
