// json3 is pre-parsed and uses the two letter attribute name + full name, so fc -> fcForeColor
// YouTube's captions.js also has both a WebVTT and json3 parser, so it's easy to see what everything means.
// https://github.com/yingted/ytcc2/blob/ece2ae8006274d7cd4aed5ac3b694f778cc9ad10/captions/src/json3.ml#L18

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Transcript;
