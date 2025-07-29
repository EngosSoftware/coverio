use serde_json::{Map, Value};

/// Badge styles as defined by `shields.io`.
enum BadgeStyle {
    Flat,
    FlatSquare,
    Plastic,
    ForTheBadge,
    Social,
}

impl BadgeStyle {
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

struct CoverageReport {
    regions_percent: f64,
    functions_percent: f64,
    lines_percent: f64,
    threshold_green: f64,
    threshold_red: f64,
    color_green: String,
    color_yellow: String,
    color_red: String,
    badge_style: BadgeStyle,
}

impl CoverageReport {
    fn new() -> Self {
        Self {
            regions_percent: 0.0,
            functions_percent: 0.0,
            lines_percent: 0.0,
            threshold_green: 80.0,
            threshold_red: 50.0,
            color_green: "21b577".to_string(),
            color_yellow: "f4b01b".to_string(),
            color_red: "f52020".to_string(),
            badge_style: BadgeStyle::Flat,
        }
    }

    fn analyse(&mut self, value: &Value) {
        let Value::Object(map) = value else {
            panic!("expected object");
        };
        let Some(data) = map.get("data") else {
            panic!("expected field 'data'");
        };
        let Value::Array(array) = data else {
            panic!("expected 'data' array");
        };
        if array.len() != 1 {
            panic!("expected single element in 'data' array");
        }
        let Some(Value::Object(map)) = array.first() else {
            panic!("expected single object in 'data' array");
        };
        let Some(totals) = map.get("totals") else {
            panic!("expected field 'totals' in 'data");
        };
        let Value::Object(totals) = totals else {
            panic!("expected 'totals' object");
        };
        self.regions_percent = self.read_percent(totals, "regions");
        self.functions_percent = self.read_percent(totals, "functions");
        self.lines_percent = self.read_percent(totals, "lines");
    }

    fn read_percent(&self, map: &Map<String, Value>, key: &str) -> f64 {
        let Some(Value::Object(map)) = map.get(key) else {
            panic!("expected '{key}' object");
        };
        let Some(Value::Number(percent_number)) = map.get("percent") else {
            panic!("expected percent field");
        };
        let Some(percent) = percent_number.as_f64() else {
            panic!("expected percent value");
        };
        percent
    }

    /// Returns a link to the `shields.io` badge reporting the coverage.
    fn badge(&self) -> String {
        let regions = self.regions_percent.trunc();
        let functions = self.functions_percent.trunc();
        let lines = self.lines_percent.trunc();
        let mut min = f64::MAX;
        for percent in [regions, functions, lines] {
            if percent < min {
                min = percent;
            }
        }
        let color = if min >= self.threshold_green {
            &self.color_green
        } else if min < self.threshold_red {
            &self.color_red
        } else {
            &self.color_yellow
        };
        let space = "%20";
        let separator = format!("{space}%E2%94%82{space}");
        let prefix = "https://img.shields.io/badge/coverage";
        let query_parameter = self.badge_style.query_parameter();
        let style = if query_parameter.is_empty() {
            "".to_string()
        } else {
            format!("?style={query_parameter}")
        };
        format!(
            "{prefix}-{regions}%25{separator}{functions}%25{separator}{lines}%25-{color}.svg{style}"
        )
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() == 2 {
        let content = std::fs::read_to_string(&args[1]).expect("failed to read input file");
        let json: Value = serde_json::from_str(&content).expect("failed to parse input JSON");
        let mut report = CoverageReport::new();
        report.analyse(&json);
        println!("  regions: {}", report.regions_percent);
        println!("functions: {}", report.functions_percent);
        println!("    lines: {}", report.lines_percent);
        println!("{}", report.badge());
    }
}
