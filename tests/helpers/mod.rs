use youtube_captions::scraper::{CaptionScraper, DigestScraper};
use youtube_captions::language_tags::LanguageTag;

pub type Any = Result<(), Box<dyn std::error::Error>>;

pub async fn with(video_id: &str, lang: &str) -> CaptionScraper {
  let scraper = DigestScraper::new(reqwest::Client::new());
  let digest = scraper.fetch(video_id, None).await.unwrap();
  let lang = LanguageTag::parse(lang).unwrap();
  digest.captions.into_iter().find(|cap| lang.matches(&cap.lang_tag)).unwrap()
}

pub const BRAVIT: &str = "JRMOIE_wAFk";
pub const SONG: &str = "PHzOOQfhPFg";
