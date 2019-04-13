pub fn update_light(current: &str) -> String {
    String::from(match current {
        "green" => "yellow",
        "yellow" => "red",
        "red" => "green",
        _ => panic!("Invalid State {}!", current),
    })
}
