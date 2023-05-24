mod error;
pub mod format;
mod scraper;

#[doc(inline)]
pub use error::{Error, Result};

pub use scraper::*;

pub mod language_tags {
  pub use language_tags::{LanguageTag, ParseError, ValidationError};
}

#[cfg(test)]
mod tests {
  use language_tags::LanguageTag;

  use crate::scraper::DigestScraper;

  type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

  #[tokio::test]
  async fn it_works() -> Result<()> {
    let video_id = "JRMOIE_wAFk";
    let scraper = DigestScraper::new(reqwest::Client::new());
    let digest = scraper.fetch(video_id, None).await?;
    let en = LanguageTag::parse("en")?;
    let en_caption = digest.captions.into_iter().find(|cap| en.matches(&cap.lang_tag)).unwrap();
    dbg!(en_caption);

    Ok(())
  }
}
