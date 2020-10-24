use std::collections::HashMap;

pub fn run() -> bool {
    println!("Analyzing...");

    println!("{:?}", analyze_string("สวัสดี"));

    true
}

pub fn analyze_string(str: &str) -> HashMap<String, u32> {
    let mut output: HashMap<String, u32> = HashMap::new();

    for ch in str.chars() {
        let counter = output.entry(ch.to_string()).or_insert(0);
        *counter += 1;
    }

    output
}

pub fn analyze_string_sorted(str: &str) -> Vec<(String, u32)> {
    let result: HashMap<String, u32> = analyze_string(&str);
    let mut result_vec: Vec<(String, u32)> = result.into_iter().collect();
    result_vec.sort_by(|a, b| b.1.cmp(&a.1));

    result_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert!(run());
    }

    #[test]
    fn test_analyze_string() {
        let output = analyze_string("123");
        assert_eq!(output["1"], 1);
        assert_eq!(output["2"], 1);
        assert_eq!(output["3"], 1);

        let output = analyze_string("1233");
        assert_eq!(output["1"], 1);
        assert_eq!(output["2"], 1);
        assert_eq!(output["3"], 2);
    }

    #[test]
    fn test_analyze_string_thai() {
        let output = analyze_string("สวัสดี");
        assert_eq!(output["ส"], 2);
        assert_eq!(output["ว"], 1);
        assert_eq!(output["ั"], 1);
        assert_eq!(output["ด"], 1);
        assert_eq!(output["ี"], 1);
    }

    #[test]
    fn test_analyze_string_sorted() {
        let output = analyze_string_sorted("544aa22a2211111");
        assert_eq!(output[0], ("1".to_string(), 5));
        assert_eq!(output[1], ("2".to_string(), 4));
        assert_eq!(output[2], ("a".to_string(), 3));
        assert_eq!(output[3], ("4".to_string(), 2));
        assert_eq!(output[4], ("5".to_string(), 1));
    }
}
