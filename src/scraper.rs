use std::collections::HashSet;

use language_tags::LanguageTag;
use regex::Regex;
use serde::Deserialize;

use crate::error::{Error, Result};
use crate::format::*;

pub struct DigestScraper {
  cookie: tokio::sync::RwLock<Option<String>>,
  http: reqwest::Client,
}

#[derive(Debug)]
pub struct Digest {
  pub captions: Vec<CaptionScraper>,
  pub can_be_translated_to: HashSet<String>,
}

#[derive(Debug, Clone)]
pub struct CaptionScraper {
  pub(crate) url: String,
  pub(crate) http: reqwest::Client,
  /// `true`, if generated using automatic speech recognition
  pub is_generated: bool,
  /// `true`, if can be translated with [`CaptionScraper::translate_to`] method.
  pub is_translatable: bool,
  pub lang_name: String,
  pub lang_tag: LanguageTag,
}

impl DigestScraper {
  pub fn new(http: reqwest::Client) -> Self {
    Self { http, cookie: tokio::sync::RwLock::new(None) }
  }

  async fn get(&self, url: &str) -> Result<String> {
    let cookie = self.cookie.read().await;
    let cookie = cookie.as_ref().map_or("", std::ops::Deref::deref);
    let res = self.http.get(url).header("Cookie", cookie).send().await?;
    Ok(res.error_for_status()?.text().await?)
  }

  async fn fetch_video_page(&self, video_id: &str, lang: &str) -> Result<String> {
    let url = format!(r#"https://youtube.com/watch?hl={}&persist_hl=1&v={}"#, lang, video_id);
    let mut html = self.get(&url).await?;

    let consent = r#"action="https://consent.youtube.com/s""#;
    if html.contains(consent) {
      let mut cookie = self.cookie.write().await;
      *cookie = Some(Self::extract_consent_cookie(&html)?);

      html = self.get(&url).await?;
      if html.contains(consent) {
        return Err(Error::FailedToCreateConsentCookie);
      }
    }

    Ok(html)
  }

  fn extract_consent_cookie(html: &str) -> Result<String, Error> {
    lazy_static::lazy_static! {
      static ref RE: Regex = Regex::new(r#"name="v" value="(.*?)"#).unwrap();
    }
    RE.captures(html)
      .ok_or(Error::FailedToCreateConsentCookie)
      .map(|caps| format!("CONSENT=YES+{};Domain=.youtube.com", &caps[1]))
  }

  fn extract_captions_json(html: &str) -> Result<RawDigest> {
    let (_, html) = html.split_once(r#""captions":"#).ok_or_else(|| match html {
      html if html.contains(r#"class="g-recaptcha""#) => Error::CaptchaRequired,
      html if !html.contains(r#""playabilityStatus":"#) => Error::VideoUnavailable,
      _ => Error::TranscriptsDisabled,
    })?;

    let (json, _) = html.split_once(r#","videoDetails"#).ok_or(Error::TranscriptsDisabled)?;
    let data: RawData = serde_json::from_str(json)?;
    Ok(data.captions)
  }

  pub async fn fetch<'a, Str: Into<Option<&'a str>>>(&self, video_id: &str, lang: Str) -> Result<Digest> {
    let lang = lang.into().unwrap_or("en");
    let html = self.fetch_video_page(video_id, lang).await?;
    let digest = DigestScraper::extract_captions_json(&html)?;

    let convert = |it: RawCaptionTrack| {
      CaptionScraper {
        url: it.base_url,
        http: self.http.clone(),
        lang_name: it.name.text,
        is_generated: matches!(it.kind.as_deref(), Some("asr")),
        is_translatable: it.is_translatable,
        lang_tag: it.language_code.parse().unwrap(), // we trust Google, don't we?
      }
    };

    Ok(Digest {
      captions: digest.caption_tracks.into_iter().map(convert).collect(),
      can_be_translated_to: digest.translation_languages.into_iter().map(|it| it.code).collect(),
    })
  }
}

impl CaptionScraper {
  /// The parameter value is an [ISO 639-1 two-letter language code] that identifies the desired caption language.
  /// The translation is generated by using machine translation, such as Google Translate.
  ///
  /// [ISO 639-1 two-letter language code]: http://www.loc.gov/standards/iso639-2/php/code_list.php
  pub fn translate_to(&mut self, language: &LanguageTag) -> Result<&mut Self> {
    if self.is_translatable {
      self.url.push_str("&tlang=");
      self.url.push_str(language.as_str());
      Ok(self)
    } else {
      Err(Error::NotTranslatable)
    }
  }

  /// The parameter specifies that the caption track should be returned in a specific format.
  /// If the parameter is not included in the request, the track is returned in its original format.
  pub async fn fetch(&self, format: Format) -> Result<String> {
    let format: &str = format.into();
    let url = format!("{}&fmt={}", &self.url, format);
    Ok(self.http.get(&url).send().await?.text().await?)
  }

  #[cfg(feature = "json3")]
  pub async fn fetch_json3(&self) -> Result<json3::Transcript> {
    unimplemented!()
  }

  #[cfg(feature = "srv1")]
  pub async fn fetch_srv1(&self) -> Result<srv1::Transcript> {
    Ok(self.fetch(Format::SRV1).await?.parse()?)
  }

  #[cfg(feature = "srv2")]
  pub async fn fetch_srv2(&self) -> Result<srv2::Transcript> {
    Ok(self.fetch(Format::SRV2).await?.parse()?)
  }

  #[cfg(feature = "srv3")]
  pub async fn fetch_srv3(&self) -> Result<srv3::Transcript> {
    Ok(self.fetch(Format::SRV3).await?.parse()?)
  }

  #[cfg(feature = "ttml")]
  pub async fn fetch_ttml(&self) -> Result<ttml::Transcript> {
    unimplemented!()
  }
}

#[derive(Debug, Deserialize)]
struct RawData {
  #[serde(rename = "playerCaptionsTracklistRenderer")]
  pub captions: RawDigest, // todo: what if the root element is absent?
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawDigest {
  pub caption_tracks: Vec<RawCaptionTrack>,
  pub translation_languages: Vec<RawLanguage>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawCaptionTrack {
  pub base_url: String,
  pub language_code: String,
  #[serde(default)]
  pub is_translatable: bool,
  #[serde(default)]
  pub kind: Option<String>,
  pub name: RawName,
}

#[derive(Debug, Deserialize)]
struct RawLanguage {
  #[serde(rename = "languageCode")]
  pub code: String,
  // #[serde(rename = "languageName")]
  // pub name: RawName,
}

#[derive(Debug, Deserialize)]
struct RawName {
  #[serde(rename = "simpleText")]
  pub text: String,
}