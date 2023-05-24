use thiserror::Error;

/// A `Result` alias where the `Err` case is `youtube_captions::Error`.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// The Errors that may occur while scraping captions.
#[derive(Debug, Error)]
pub enum Error {
  #[error("the video is no longer available")]
  VideoUnavailable,

  /// YouTube is receiving too many requests from this IP and now requires solving a captcha to continue.
  /// One of the following things can be done to work around this:
  /// - Manually solve the captcha in a browser and export the cookie
  /// - Use a different IP address
  /// - Wait until the ban on your IP has been lifted
  #[error(
    "YouTube is receiving too many requests from this IP and now requires solving a captcha to continue.
  One of the following things can be done to work around this:
  - Manually solve the captcha in a browser and export the cookie
  - Use a different IP address
  - Wait until the ban on your IP has been lifted"
  )]
  CaptchaRequired,

  /// Subtitles are disabled for this video.
  #[error("Subtitles are disabled for this video")]
  TranscriptsDisabled,

  // /// No transcripts are available for this video.
  // #[error("No transcripts are available for this video")]
  // NoTranscriptAvailable,

  /// The requested file is not translatable.
  #[error("The requested file is not translatable")]
  NotTranslatable,

  /// The requested translation language is not available.
  #[error("The requested translation language is not available")]
  TranslationLanguageNotAvailable,

  /// The cookies provided are not valid (may have expired).
  #[error("The cookies provided are not valid (may have expired)")]
  CookiesInvalid,

  /// Failed to automatically give consent to saving cookies.
  #[error("Failed to automatically give consent to saving cookies")]
  FailedToCreateConsentCookie,

  #[error("Request to YouTube failed: {0}")]
  NetworkError(#[from] reqwest::Error),

  #[error("Invalid JSON: {0}")]
  InvalidJson(#[from] serde_json::Error),

  #[cfg(feature = "quick-xml")]
  #[error("Invalid XML: {0}")]
  InvalidXml(#[from] quick_xml::de::DeError),
}
