#[cfg(feature = "json3")]
pub mod json3;
#[cfg(feature = "srv1")]
pub mod srv1;
#[cfg(feature = "srv2")]
pub mod srv2;
#[cfg(feature = "srv3")]
pub mod srv3;
#[cfg(feature = "ttml")]
pub mod ttml;

#[derive(Debug, Default, Clone, Copy)]
pub enum Format {
  /// Web Video Text Tracks
  VTT,

  /// Timed Text Markup Language
  TTML,

  /// YouTube Timed Text, v1
  #[default]
  SRV1,

  /// YouTube Timed Text, v2
  SRV2,

  /// YouTube Timed Text, v3
  SRV3,

  JSON3,
}

impl From<Format> for &'static str {
  fn from(value: Format) -> Self {
    match value {
      Format::VTT => "vtt",
      Format::TTML => "ttml",
      Format::SRV1 => "srv1",
      Format::SRV2 => "srv2",
      Format::SRV3 => "srv3",
      Format::JSON3 => "json3",
    }
  }
}
