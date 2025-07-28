use serde_json::{Map, Value};

struct Visitor {
    //
}

impl Visitor {
    fn new() -> Self {
        Self {}
    }

    fn visit(&self, value: &Value) {
        let Value::Object(map) = value else {
            panic!("expected object");
        };
        let Some(data) = map.get("data") else {
            panic!("expected field 'data'");
        };
        let Value::Array(array) = data else {
            panic!("expected 'data' array");
        };
        assert_eq!(1, array.len());
        let Some(Value::Object(map)) = array.first() else {
            panic!("expected object");
        };
        let Some(totals) = map.get("totals") else {
            panic!("expected field 'totals' in 'data");
        };
        let Value::Object(totals) = totals else {
            panic!("expected 'totals' object");
        };
        let functions_percent = Self::get_percent(totals, "functions");
        println!("functions: {functions_percent}");
        let lines_percent = Self::get_percent(totals, "lines");
        println!("    lines: {lines_percent}");
        let regions_percent = Self::get_percent(totals, "regions");
        println!("  regions: {regions_percent}");
    }

    fn get_percent(map: &Map<String, Value>, key: &str) -> f64 {
        let Some(Value::Object(map)) = map.get(key) else {
            panic!("expected '{key}' object");
        };
        let Some(Value::Number(percent_number)) = map.get("percent") else {
            panic!("expected percent number");
        };
        let Some(percent) = percent_number.as_f64() else {
            panic!("expected percent value");
        };
        (percent * 100.0).trunc() / 100.0
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() == 2 {
        let content = std::fs::read_to_string(&args[1]).expect("failed to read input file");
        let json: Value = serde_json::from_str(&content).expect("failed to parse input JSON");
        let visitor = Visitor::new();
        visitor.visit(&json);
    }
}
