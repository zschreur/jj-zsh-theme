const CMD: char = '\u{f4b5}';
const CWD: char = '\u{f07c}';

// https://man.archlinux.org/man/zshmisc.1#SIMPLE_PROMPT_ESCAPES
// https://wiki.archlinux.org/title/Zsh#Customized_prompt

fn change_id_for_revision(rev: &str) -> Result<String, Box<dyn std::error::Error>> {
    let output = std::process::Command::new("jj")
        .arg("log")
        .arg("-r")
        .arg(rev)
        .arg("-T")
        .arg("change_id.shortest()")
        .arg("--no-pager")
        .arg("--no-graph")
        .output()?;

    if !output.status.success() {
        return Err("jj log failed".into());
    }

    Ok(String::from_utf8(output.stdout).map(|s| s.trim().to_string())?)
}

fn change_id() -> Result<String, Box<dyn std::error::Error>> {
    change_id_for_revision("@")
}

fn parent_change() -> Option<String> {
    match change_id_for_revision("@+") {
        Ok(c) => Some(c),
        _ => None,
    }
}

fn write_init() {
    print!("%F{{cyan}}{} %f", CMD); // Command symbol
}

fn write_cwd() {
    print!("%F{{default}}[%~ {} ]%f ", CWD); // Current working directory, with 2 components
}

fn main() {
    write_init();
    write_cwd();
    match change_id() {
        Ok(change_id) => print!("%F{{127}}{} %f", change_id),
        Err(_) => (),
    }
    match parent_change() {
        Some(parent_change) => print!("%F{{127}}{} %f", parent_change),
        None => (),
    }
}