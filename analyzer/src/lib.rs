use std::collections::HashMap;

pub fn run() -> bool {
    println!("Analyzing...");

    println!("{:?}", analyze_string("สวัสดี"));

    true
}

fn analyze_string(str: &str) -> HashMap<String, u32> {
    let mut output: HashMap<String, u32> = HashMap::new();

    for ch in str.chars() {
        let counter = output.entry(ch.to_string()).or_insert(0);
        *counter += 1;
    }

    output
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
}
