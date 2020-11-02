use std::collections::HashMap;

pub fn analyze_string(str: &str) -> HashMap<String, u32> {
    let mut output: HashMap<String, u32> = HashMap::new();

    for ch in str.chars() {
        match ch {
            '\n' => continue,
            ' ' => continue,
            '0'..='9' => continue,
            _ => {
                let counter = output.entry(ch.to_string()).or_insert(0);
                *counter += 1;
            },
        }
    }

    output
}

pub fn analyze_string_sorted(str: &str) -> Vec<(String, u32)> {
    let result: HashMap<String, u32> = analyze_string(&str);
    let mut result_vec: Vec<(String, u32)> = result.into_iter().collect();
    result_vec.sort_by(|a, b| b.1.cmp(&a.1));

    result_vec
}

fn analyze_string_triads(str: &str) -> HashMap<String, u32> {
    let mut output: HashMap<String, u32> = HashMap::new();

    let strArray = str.chars().collect();

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_string() {
        let output = analyze_string("กขค");
        assert_eq!(output["ก"], 1);
        assert_eq!(output["ข"], 1);
        assert_eq!(output["ค"], 1);

        let output = analyze_string("กขคค");
        assert_eq!(output["ก"], 1);
        assert_eq!(output["ข"], 1);
        assert_eq!(output["ค"], 2);
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
    fn test_analyze_string_skip_whitespaces() {
        let output = analyze_string("สวัสดี \n ลาก่อน");
        assert_eq!(output.get(" "), None);
        assert_eq!(output.get("\n"), None);
    }

    #[test]
    fn test_analyze_string_sorted() {
        let output = analyze_string_sorted("จ งง ขขขข คคค กกกกก");
        assert_eq!(output[0], ("ก".to_string(), 5));
        assert_eq!(output[1], ("ข".to_string(), 4));
        assert_eq!(output[2], ("ค".to_string(), 3));
        assert_eq!(output[3], ("ง".to_string(), 2));
        assert_eq!(output[4], ("จ".to_string(), 1));
    }

    #[test]
    fn test_analyze_string_triads() {
        let output = analyze_string_triads("abcdefg hij kl m bcde");

        assert_eq!(output["abc"], 1);
    }
}
