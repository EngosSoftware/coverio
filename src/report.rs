use crate::errors::{CoverioError, Result};
use serde_json::{Map, Value};

/// Unicode character ` ` (U+0020) Space (SP)
const SPACE: &str = "%20";

/// Unicode character `%` (U+0025) Percent Sign
const PERCENT: &str = "%25";

/// Unicode character `│` (U+2502) Box Drawings Light Vertical
const BOX_DRAWINGS_LIGHT_VERTICAL: &str = "%E2%94%82";

/// Unicode character `—` (U+2014) Em Dash.
const EM_DASH: &str = "%E2%80%94";

/// Badge styles as defined by `shields.io`.
#[derive(Debug)]
pub enum BadgeStyle {
  /// `flat` style.
  Flat,
  /// `flat-square` style.
  FlatSquare,
  /// `plastic` style.
  Plastic,
  /// `for-the-badge` style.
  ForTheBadge,
  /// `social` style.
  Social,
}

impl BadgeStyle {
  /// Returns a query parameter based on badge style.
  fn query_parameter(&self) -> &'static str {
    match self {
      BadgeStyle::Flat => "",
      BadgeStyle::FlatSquare => "flat-square",
      BadgeStyle::Plastic => "plastic",
      BadgeStyle::ForTheBadge => "for-the-badge",
      BadgeStyle::Social => "social",
    }
  }
}

impl From<&str> for BadgeStyle {
  fn from(value: &str) -> Self {
    match value {
      "flat" => Self::Flat,
      "flat-square" => Self::FlatSquare,
      "plastic" => Self::Plastic,
      "for-the-badge" => Self::ForTheBadge,
      "social" => Self::Social,
      _ => Self::Flat,
    }
  }
}

pub enum SeparatorStyle {
  /// Single space.
  Space,
  /// Vertical bar.
  Bar,
  /// Vertical bar with space before and after.
  SpacedBar,
}

impl From<&str> for SeparatorStyle {
  fn from(value: &str) -> Self {
    match value {
      "space" => Self::Space,
      "bar" => Self::Bar,
      "spaced-bar" => Self::SpacedBar,
      _ => Self::SpacedBar,
    }
  }
}

pub struct CoverageReport {
  /// Total number of regions.
  regions_count: u64,
  /// Percent of regions covered.
  regions_percent: f64,
  /// Total number of functions.
  functions_count: u64,
  /// Percent of functions covered.
  functions_percent: f64,
  /// Total number of lines.
  lines_count: u64,
  /// Percent of lines covered.
  lines_percent: f64,
  /// Threshold where high code coverage starts.
  threshold_high: f64,
  /// Threshold where moderate code coverage starts.
  threshold_moderate: f64,
  /// Color used for high code coverage.
  color_high: String,
  /// Color used for moderate code coverage.
  color_moderate: String,
  /// Color used for low code coverage.
  color_low: String,
  /// Flag indicating if the percent sign should be visible.
  show_percent_sign: bool,
}

impl CoverageReport {
  pub fn new() -> Self {
    Self {
      regions_count: 0,
      regions_percent: 0.0,
      functions_count: 0,
      functions_percent: 0.0,
      lines_count: 0,
      lines_percent: 0.0,
      threshold_high: 80.0,
      threshold_moderate: 50.0,
      color_high: "21b577".to_string(),
      color_moderate: "f4b01b".to_string(),
      color_low: "f52020".to_string(),
      show_percent_sign: true,
    }
  }

  pub fn regions_percent(&self) -> f64 {
    self.regions_percent
  }

  pub fn functions_percent(&self) -> f64 {
    self.functions_percent
  }

  pub fn lines_percent(&self) -> f64 {
    self.lines_percent
  }

  pub fn analyze(&mut self, value: &Value) -> Result<(), CoverioError> {
    let Value::Object(map) = value else {
      return Err(CoverioError::new("expected object"));
    };
    let Some(data) = map.get("data") else {
      return Err(CoverioError::new("expected field 'data'"));
    };
    let Value::Array(array) = data else {
      return Err(CoverioError::new("expected 'data' array"));
    };
    if array.len() != 1 {
      return Err(CoverioError::new("expected single element in 'data' array"));
    }
    let Some(Value::Object(map)) = array.first() else {
      return Err(CoverioError::new("expected single object in 'data' array"));
    };
    let Some(totals) = map.get("totals") else {
      return Err(CoverioError::new("expected field 'totals' in 'data` object"));
    };
    let Value::Object(totals) = totals else {
      return Err(CoverioError::new("expected 'totals' object"));
    };
    (self.regions_count, self.regions_percent) = self.read_values(totals, "regions")?;
    (self.functions_count, self.functions_percent) = self.read_values(totals, "functions")?;
    (self.lines_count, self.lines_percent) = self.read_values(totals, "lines")?;
    Ok(())
  }

  fn read_values(&self, map: &Map<String, Value>, key: &str) -> Result<(u64, f64), CoverioError> {
    let Some(Value::Object(map)) = map.get(key) else {
      return Err(CoverioError::new(format!("expected '{key}' object")));
    };
    let Some(Value::Number(count_number)) = map.get("count") else {
      return Err(CoverioError::new("expected 'count' number"));
    };
    let Some(count) = count_number.as_u64() else {
      return Err(CoverioError::new("invalid 'count' number"));
    };
    let Some(Value::Number(percent_number)) = map.get("percent") else {
      return Err(CoverioError::new("expected 'percent' number"));
    };
    let Some(percent) = percent_number.as_f64() else {
      return Err(CoverioError::new("invalid 'percent' number"));
    };
    Ok((count, percent))
  }

  /// Returns a link of the `shields.io` badge reporting the coverage.
  pub fn badge(&self, badge_style: BadgeStyle, badge_label: String, separator_style: SeparatorStyle) -> String {
    let regions_perc = self.regions_percent.trunc();
    let functions_perc = self.functions_percent.trunc();
    let lines_perc = self.lines_percent.trunc();
    let mut min = f64::MAX;
    for (percent, count) in [regions_perc, functions_perc, lines_perc]
      .iter()
      .zip([self.regions_count, self.functions_count, self.lines_count])
    {
      if *percent < min && count > 0 {
        min = *percent;
      }
    }
    let color = if min >= self.threshold_high {
      &self.color_high
    } else if min < self.threshold_moderate {
      &self.color_low
    } else {
      &self.color_moderate
    };
    let separator = self.separator(separator_style);
    let prefix = format!("https://img.shields.io/badge/{}", badge_label);
    let query_parameter = badge_style.query_parameter();
    let style = if query_parameter.is_empty() {
      "".to_string()
    } else {
      format!("?style={query_parameter}")
    };
    let regions = self.label_value(regions_perc, self.regions_count);
    let functions = self.label_value(functions_perc, self.functions_count);
    let lines = self.label_value(lines_perc, self.lines_count);
    format!("{prefix}-{regions}{separator}{functions}{separator}{lines}-{color}.svg{style}")
  }

  /// Returns the coverage values separator.
  fn separator(&self, separator_style: SeparatorStyle) -> String {
    match separator_style {
      SeparatorStyle::Space => SPACE.to_string(),
      SeparatorStyle::Bar => BOX_DRAWINGS_LIGHT_VERTICAL.to_string(),
      SeparatorStyle::SpacedBar => format!("{SPACE}{BOX_DRAWINGS_LIGHT_VERTICAL}{SPACE}"),
    }
  }

  /// Returns the formatted coverage value.
  fn label_value(&self, value: f64, count: u64) -> String {
    if count > 0 {
      if self.show_percent_sign { format!("{value}{PERCENT}") } else { format!("{value}") }
    } else {
      EM_DASH.to_string()
    }
  }
}
