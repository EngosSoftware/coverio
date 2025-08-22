use crate::errors::{CoverioError, Result};
use serde_json::{Map, Value};

/// Badge styles as defined by `shields.io`.
#[derive(Debug)]
pub enum BadgeStyle {
  Default,
  Flat,
  FlatSquare,
  Plastic,
  ForTheBadge,
  Social,
}

impl BadgeStyle {
  fn query_parameter(&self) -> &'static str {
    match self {
      BadgeStyle::Default => "",
      BadgeStyle::Flat => "flat",
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
      _ => Self::Default,
    }
  }
}

pub struct CoverageReport {
  regions_count: u64,
  regions_percent: f64,
  functions_count: u64,
  functions_percent: f64,
  lines_count: u64,
  lines_percent: f64,
  threshold_green: f64,
  threshold_red: f64,
  color_green: String,
  color_yellow: String,
  color_red: String,
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
      threshold_green: 80.0,
      threshold_red: 50.0,
      color_green: "21b577".to_string(),
      color_yellow: "f4b01b".to_string(),
      color_red: "f52020".to_string(),
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
  pub fn badge(&self, badge_style: BadgeStyle) -> String {
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
    let color = if min >= self.threshold_green {
      &self.color_green
    } else if min < self.threshold_red {
      &self.color_red
    } else {
      &self.color_yellow
    };
    let separator = separator();
    let prefix = "https://img.shields.io/badge/coverage";
    let query_parameter = badge_style.query_parameter();
    let style = if query_parameter.is_empty() {
      "".to_string()
    } else {
      format!("?style={query_parameter}")
    };
    let regions = label_value(regions_perc, self.regions_count);
    let functions = label_value(functions_perc, self.functions_count);
    let lines = label_value(lines_perc, self.lines_count);
    format!("{prefix}-{regions}{separator}{functions}{separator}{lines}-{color}.svg{style}")
  }
}

fn separator() -> String {
  let space = "%20";
  format!("{space}%E2%94%82{space}")
}

fn label_value(percent: f64, count: u64) -> String {
  const DASH: &str = "%E2%80%94";
  if count > 0 { format!("{percent}%25") } else { DASH.to_string() }
}
