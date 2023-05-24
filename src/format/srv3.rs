use std::borrow::Cow;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

use quick_xml::escape::unescape as unescape_xml;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

// todo: https://github.com/arcusmaximus/YTSubConverter#ass-feature-support
// todo: https://jacobstar.medium.com/the-first-complete-guide-to-youtube-captions-f886e06f7d9d
// todo: https://github.com/arcusmaximus/YTSubConverter/blob/master/ytt.ytt

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Transcript {
  pub head: Head,
  pub body: Body,
  #[serde(rename = "@format")]
  pub format_version: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Head {
  #[serde(default, rename = "pen")]
  pub pens: Vec<Pen>,
  #[serde(default, rename = "ws")]
  pub window_styling: Vec<WindowStyle>,
  #[serde(default, rename = "wp")]
  pub window_positioning: Vec<WindowPosition>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Pen {
  #[serde(rename = "@id")]
  pub id: u32,
  #[serde(default, rename = "@b")]
  pub bold: bool,
  #[serde(default, rename = "@i")]
  pub italic: bool,
  #[serde(default, rename = "@u")]
  pub underline: bool,

  #[serde(default, rename = "@fc")]
  pub foreground_color: Option<String>,
  #[serde(default, rename = "@fo")]
  pub foreground_opacity: Option<u8>,

  #[serde(default, rename = "@bc")]
  pub background_color: Option<String>,
  #[serde(default, rename = "@bo")]
  pub background_opacity: Option<u8>,

  #[serde(default, rename = "@ec")]
  pub edge_color: Option<String>,
  #[serde(default, rename = "@et")]
  pub edge_type: EdgeType,

  /// Android doesn't support custom fonts at all, while iOS "supports" them as in using non-default
  /// fonts which however look completely different than on PC.
  #[serde(default, rename = "@fs")]
  pub font_family: FontStyle,
  /// The value is a virtual percentage of the default size.
  ///
  /// The real percentage is `100 + (sz - 100) / 4`, meaning `sz="200"` will result
  /// in text that's *not* twice as big as the default but only 25% bigger.
  ///
  /// The values can't be negative, meaning the smallest you can go is a
  /// virtual percentage of 0 which equates to a real percentage of 75.
  ///
  /// Supported on iOS but not Android.
  #[serde(default, rename = "@si")]
  pub font_size_perc: Option<u32>,

  /// Vertical text alignment. Not supported on mobile devices.
  #[serde(default, rename = "@of")]
  pub vertical_alignment: Option<VerticalAlignment>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct WindowStyle {
  #[serde(rename = "@id")]
  pub id: u32,
  #[deprecated(note = "Raise an issue, if you know how to interpret the values")]
  #[serde(default, rename = "@sd")]
  pub scroll_direction: u8,
  #[serde(default, rename = "@pd")]
  pub print_direction: PrintDirection,
  #[serde(rename = "@ju")]
  pub text_alignment: Option<TextAlignment>,
  #[serde(default, rename = "@mh")]
  pub mode_hint: ModeHint,
  #[serde(default, rename = "@wfc")]
  pub fill_color: Option<String>,
  #[serde(default, rename = "@wfo")]
  pub fill_opacity: Option<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct WindowPosition {
  #[serde(rename = "@id")]
  pub id: u32,
  /// Point on the subtitle box
  ///
  /// <pre>
  /// 0 ======== 1 ======== 2
  /// |                     |
  /// 3          4          5
  /// |                     |
  /// 6 ======== 7 ======== 8
  /// </pre>
  #[serde(rename = "@ap")]
  pub anchor_point: Option<AnchorPoint>,
  /// The player transforms the coordinates according to `effectiveCoord = (specifiedCoord * 0.96) + 2`,
  /// meaning subtitles don't appear *quite* where you want them to.
  ///
  /// In theater mode, the width covered by `left_offset` includes the black bars on the sides, meaning the
  /// subtitles move towards the sides and even out of the video.
  #[serde(default, rename = "@ah")]
  pub left_offset: Option<u32>,
  /// The player transforms the coordinates according to `effectiveCoord = (specifiedCoord * 0.96) + 2`,
  /// meaning subtitles don't appear *quite* where you want them to.
  ///
  /// In theater mode, the width covered by `top_offset` includes the black bars on the sides, meaning the
  /// subtitles move towards the sides and even out of the video.
  #[serde(default, rename = "@av")]
  pub top_offset: Option<u32>,
  #[serde(default, rename = "@rc")]
  pub rows_total: Option<u8>,
  /// Each column has en-dash width
  #[serde(default, rename = "@cc")]
  pub columns_total: Option<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Body {
  #[serde(rename = "$value")]
  pub elements: Vec<Element>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Element {
  #[serde(rename = "p")]
  Segment(TextSegment),
  #[serde(rename = "w")]
  Window(Window),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct TextSegment {
  #[serde(rename = "@t")]
  pub time_millis: u32,
  #[serde(default, rename = "@d")]
  pub duration_millis: u32,
  #[serde(default, rename = "@p")]
  pub pen_id: Option<u32>,
  #[serde(default, rename = "@wp")]
  pub window_position_id: Option<u32>,
  #[serde(default, rename = "@ws")]
  pub window_style_id: Option<u32>,
  #[serde(default, rename = "$value")]
  pub value: Vec<Text>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Window {
  #[serde(rename = "@id")]
  pub id: u32,
  #[serde(rename = "@t")]
  pub time_millis: u32,
  #[serde(rename = "@wp")]
  pub window_position_id: u32,
  #[serde(rename = "@ws")]
  pub window_style_id: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Text {
  #[serde(rename = "s")]
  Span(Span),
  #[serde(rename = "$text")]
  Str(String),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Span {
  #[serde(default, rename = "@t")]
  pub relative_time_millis: u32,
  #[serde(rename = "@p")]
  pub pen_id: Option<u32>,
  #[serde(rename = "$value")]
  pub value: String,
}

/// <pre>
/// 0 ======== 1 ======== 2
/// |                     |
/// 3          4          5
/// |                     |
/// 6 ======== 7 ======== 8
/// </pre>
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum AnchorPoint {
  TopLeft = 0,
  Top = 1,
  TopRight = 2,
  Left = 3,
  Center = 4,
  Right = 5,
  BottomLeft = 6,
  Bottom = 7,
  BottomRight = 8,
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
pub enum EdgeType {
  #[default]
  None = 0,
  HardShadow = 1,
  Bevel = 2,
  GlowOutline = 3,
  SoftShadow = 4,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum FontStyle {
  /// Same as [FontStyle::ProportionalSansSerif]
  #[default]
  Default,
  /// Courier New
  MonospacedSerif,
  /// Times New Roman
  ProportionalSerif,
  /// Lucida Console
  MonospacedSansSerif,
  /// Roboto
  ProportionalSansSerif,
  /// Comic Sans
  Casual,
  /// Monotype Corsiva
  Cursive,
  /// Arial with font-variant: small-caps
  SmallCapitals,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum VerticalAlignment {
  Subscript = 0,
  #[default]
  Regular = 1,
  Superscript = 2,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum RubyPart {
  #[default]
  None = 0,
  /// for kanji spans
  Base = 1,
  /// for clients that don't support ruby
  Parenthesis = 2,
  // todo: 3?
  /// for furigana spans
  TextBefore = 4,
  /// for furigana spans
  TextAfter = 5,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ModeHint {
  #[default]
  None = 0,
  Default = 1,
  Scroll = 2,
}

impl Deref for Text {
  type Target = String;

  fn deref(&self) -> &Self::Target {
    match self {
      Text::Span(span) => &span.value,
      Text::Str(value) => &value,
    }
  }
}

impl DerefMut for Text {
  fn deref_mut(&mut self) -> &mut Self::Target {
    match self {
      Text::Span(span) => &mut span.value,
      Text::Str(ref mut value) => value,
    }
  }
}

impl AsRef<str> for Text {
  fn as_ref(&self) -> &str {
    self.deref().as_ref()
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

    for it in &mut transcript.body.elements {
      if let Element::Segment(seg) = it {
        for it in &mut seg.value {
          if let unescaped @ Cow::Owned(_) = unescape_xml(it).unwrap() {
            *it.deref_mut() = unescaped.into_owned();
          };
        }
      }
    }

    Ok(transcript)
  }
}
