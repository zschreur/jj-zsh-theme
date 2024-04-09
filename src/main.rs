const CMD: char = '\u{f4b5}';
const CWD: char = '\u{f07c}';
const MID_COMMIT: char = '\u{eafc}';
const LEAF: char = '\u{f032a}';

// https://man.archlinux.org/man/zshmisc.1#SIMPLE_PROMPT_ESCAPES
// https://wiki.archlinux.org/title/Zsh#Customized_prompt

fn change_id_for_revision(rev: &str) -> Result<(String, String), Box<dyn std::error::Error>> {
    let output = std::process::Command::new("jj")
        .arg("log")
        .arg("-r")
        .arg(rev)
        .arg("-T")
        .arg(r#"concat(change_id.shortest(8).prefix(), "\n", change_id.shortest(8).rest())"#)
        .arg("--no-pager")
        .arg("--no-graph")
        .arg("--ignore-working-copy")
        .arg("--quiet")
        .output()?;

    if !output.status.success() {
        return Err("jj log failed".into());
    }

    let output = String::from_utf8(output.stdout)?;
    let output: Vec<&str> = output.trim().split('\n').collect();
    let prefix = output.get(0).unwrap_or(&"").to_string();
    let suffix = output.get(1).unwrap_or(&"").to_string();
    Ok((prefix, suffix))
}

fn change_id() -> Result<(String, String), Box<dyn std::error::Error>> {
    change_id_for_revision("@")
}

fn child() -> Option<(String, String)> {
    match change_id_for_revision("@+") {
        Ok(c) => Some(c),
        _ => None,
    }
}

fn write_init() {
    print!("%F{{38}}{} %f", CMD);
}

fn write_cwd() {
    print!("%F{{246}}[%~ {} ]%f ", CWD);
}

fn main() {
    write_init();
    write_cwd();
    let has_child = match child() {
        Some((child_change, _)) => !child_change.is_empty(),
        _ => false,
    };
    match change_id() {
        Ok(change_id) => {
            print!("( %B%F{{magenta}}{}%f%b%F{{244}}{} %f", change_id.0, change_id.1);
            if has_child {
                print!("%F{{202}}{}", MID_COMMIT)
            } else {
                print!("%F{{green}}{}", LEAF)
            }
            print!("%f ) ")
        },
        Err(_) => (),
    }
}
