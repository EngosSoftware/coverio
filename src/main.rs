use serde_json::{Map, Value};

struct CoverageReport {
    functions_percent: f64,
    lines_percent: f64,
    regions_percent: f64,
}

impl CoverageReport {
    fn new() -> Self {
        Self {
            functions_percent: 0.0,
            lines_percent: 0.0,
            regions_percent: 0.0,
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
        self.functions_percent = self.read_percent(totals, "functions");
        self.lines_percent = self.read_percent(totals, "lines");
        self.regions_percent = self.read_percent(totals, "regions");
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
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() == 2 {
        let content = std::fs::read_to_string(&args[1]).expect("failed to read input file");
        let json: Value = serde_json::from_str(&content).expect("failed to parse input JSON");
        let mut report = CoverageReport::new();
        report.analyse(&json);
        println!("functions: {}", report.functions_percent);
        println!("    lines: {}", report.lines_percent);
        println!("  regions: {}", report.regions_percent);
    }
}
