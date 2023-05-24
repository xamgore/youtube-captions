use std::borrow::Cow;
use std::str::FromStr;

use quick_xml::escape::unescape as unescape_xml;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transcript {
  #[serde(rename(deserialize = "text"))]
  pub segments: Vec<TextSegment>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextSegment {
  #[serde(rename(deserialize = "@start"))]
  pub start_secs: f32,
  #[serde(rename(deserialize = "@dur"))]
  pub duration_secs: f32,
  #[serde(rename(deserialize = "$value"))]
  pub value: String,
}

impl TextSegment {
  pub fn end_secs(&self) -> f32 {
    self.start_secs + self.duration_secs
  }
}

impl IntoIterator for Transcript {
  type Item = TextSegment;
  type IntoIter = std::vec::IntoIter<Self::Item>;

  fn into_iter(self) -> Self::IntoIter {
    self.segments.into_iter()
  }
}

impl FromStr for Transcript {
  type Err = quick_xml::DeError;

  fn from_str(input: &str) -> Result<Self, Self::Err> {
    #[cfg(not(test))]
    let mut transcript: Transcript = quick_xml::de::from_str(input)?;

    #[cfg(test)]
    let mut transcript: Transcript = {
      let de = &mut quick_xml::de::Deserializer::from_str(input);
      serde_path_to_error::deserialize(de).unwrap()
    };

    for it in &mut transcript.segments {
      if let unescaped @ Cow::Owned(_) = unescape_xml(&it.value).unwrap() {
        it.value = unescaped.into_owned();
      };
    }

    Ok(transcript)
  }
}
