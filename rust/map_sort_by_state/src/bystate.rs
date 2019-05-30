use std::collections::HashMap

#[allow(dead_code)]
pub fn by_state(str: &str) -> String {
    let state_full_name = HashMap::new();
    state_full_name.insert("AZ".to_string(), "Arizona".to_string());
    state_full_name.insert("CA".to_string(), "California".to_string());
    state_full_name.insert("ID".to_string(), "Idaho".to_string());
    state_full_name.insert("IN".to_string(), "Indiana".to_string());
    state_full_name.insert("MA".to_string(), "Massachusetts".to_string());
    state_full_name.insert("OK".to_string(), "Oklahoma".to_string());
    state_full_name.insert("PA".to_string(), "Pennsylvania".to_string());
    state_full_name.insert("VA".to_string(), "Virginia".to_string());
    
    let mut state_visitors: HashMap<String, Vec<String>> = HashMap::new();
}
