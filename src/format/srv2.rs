use std::borrow::Cow;
use std::str::FromStr;

use quick_xml::escape::unescape as unescape_xml;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Transcript {
  #[serde(rename = "$value")]
  pub elements: Vec<Element>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Element {
  Text(TextSegment),
  Window(Window),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct TextSegment {
  #[serde(rename(deserialize = "@t"))]
  pub timestamp_millis: u32,
  #[serde(default, rename(deserialize = "@d"))]
  pub duration_millis: u32,
  #[serde(default, rename(deserialize = "@append"))]
  pub append: bool,
  #[deprecated(note = "if you know what this field is about, please, raise an issue")] // todo: paste a link to github
  #[serde(default, rename(deserialize = "@r"))]
  pub r: u32,
  #[deprecated(note = "if you know what this field is about, please, raise an issue")] // todo: paste a link to github
  #[serde(default, rename(deserialize = "@c"))]
  pub c: u32,
  #[serde(default, rename(deserialize = "$value"))]
  pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Window {
  #[serde(rename(deserialize = "@id"))]
  pub id: u32,
  // todo: may have values "define", ...?
  #[serde(rename(deserialize = "@op"))]
  pub operation: String,
  #[serde(rename(deserialize = "@t"))]
  pub timestamp_millis: u32,
  #[serde(rename(deserialize = "@ap"))]
  pub anchor_point: AnchorPoint,
  /// X from left
  #[serde(rename(deserialize = "@ah"))]
  pub horizontal_alignment: u32,
  /// Y from top
  #[serde(rename(deserialize = "@av"))]
  pub vertical_alignment: u32,
  #[serde(rename(deserialize = "@rc"))]
  pub rows_total: u8,
  /// Each column has en-dash width
  #[serde(rename(deserialize = "@cc"))]
  pub columns_total: u8,
  #[serde(rename(deserialize = "@sd"))]
  pub scroll_direction: ScrollDirection,
  #[serde(default, rename(deserialize = "@pd"))]
  pub print_direction: PrintDirection,
  #[serde(rename(deserialize = "@ju"))]
  pub text_alignment: TextAlignment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum AnchorPoint {
  TopLeft = 0,
  TopCenter = 1,
  TopRight = 2,
  CenterLeft = 3,
  TrueCenter = 4,
  CenterRight = 5,
  BottomLeft = 6,
  BottomCenter = 7,
  BottomRight = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum TextAlignment {
  /// Equals to _left_ in LTR
  Start = 0,
  /// Equals to _right_ in LTR
  Eng = 1,
  Center = 2,
  Justify = 3,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum PrintDirection {
  #[default]
  LtrHorizontal = 0,
  RtlHorizontal = 1,
  /// Upright text: lays out the glyphs for vertical scripts naturally (upright),
  /// as well as the characters of horizontal scripts
  VerticalLtr = 2,
  /// Sideways text: causes characters to be laid out as they would be horizontally,
  /// but with the whole line rotated 90° clockwise.
  VerticalRtl = 3,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ScrollDirection {
  #[default]
  LTR = 0,
  RTL = 1,
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

    for it in &mut transcript.elements {
      if let Element::Text(it) = it {
        if let unescaped @ Cow::Owned(_) = unescape_xml(&it.value).unwrap() {
          it.value = unescaped.into_owned();
        };
      }
    }

    Ok(transcript)
  }
}
