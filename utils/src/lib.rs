use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Loc<T> {
    pub x: T,
    pub y: T,
}

/// Gets the input for the puzzle
pub fn get_input(project_dir: &str, is_example: bool) -> String {
    let path = project_dir.to_string()
        + "/../inputs/input-"
        + &project_dir[project_dir.len() - 2..] // XX in /home/.../dayXX
        + (if is_example { "-ex" } else { "" })
        + ".txt";
    fs::read_to_string(path).unwrap()
}
