use std::fs::read_to_string;

pub fn cwd() -> String {
    std::env::current_dir().unwrap().display().to_string()
}

pub fn read_file(day: &str, example: bool) -> String {
    let example_part = match example {
        true => "example",
        _ => "",
    };
    let path = format!("{}/inputs/{}{}.txt", cwd(), day, example_part);
    read_to_string(path).unwrap()
}
