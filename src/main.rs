const INIT: char = '\u{f01e5}';
const CWD: char = '\u{f07c}';
const PARENT_COMMIT: char = '\u{eafc}';
const LEAF: char = '\u{f032a}';

fn write_init() {
    print!("%F{{38}}{} %f", INIT);
}

// https://man.archlinux.org/man/zshmisc.1#SIMPLE_PROMPT_ESCAPES
// https://wiki.archlinux.org/title/Zsh#Customized_prompt

struct JJStatus {
    change_id: (String, String),
    child_change_id: Option<(String, String)>,
}

fn get_jj_status() -> Result<JJStatus, Box<dyn std::error::Error>> {
    let output = std::process::Command::new("jj")
        .arg("log")
        .arg("-r")
        .arg("@::")
        .arg("--limit=2")
        .arg("--reversed")
        .arg("-T")
        .arg(r#"concat(change_id.shortest(8).prefix(), "-", change_id.shortest(8).rest(), "\n")"#)
        .arg("--no-pager")
        .arg("--no-graph")
        .arg("--ignore-working-copy")
        .arg("--quiet")
        .output()?;

    if !output.status.success() {
        return Err("jj command failed".into());
    }

    let output = String::from_utf8(output.stdout)?;
    let output: Vec<(String, String)> = output
        .trim()
        .split('\n')
        .map(|line| {
            let parts: Vec<&str> = line.split('-').collect();
            (parts[0].to_string(), parts[1].to_string())
        })
        .collect();

    Ok(JJStatus {
        change_id: output[0].to_owned(),
        child_change_id: output.get(1).map(|x| x.to_owned()),
    })
}

fn write_cwd() {
    print!("%F{{246}}[%~ {} ]%f ", CWD);
}

fn is_in_jj_repo() -> bool {
    std::path::Path::new(".jj").exists()
}

fn main() {
    write_init();
    write_cwd();
    if !is_in_jj_repo() {
        return;
    }

    let jj_status = match get_jj_status() {
        Ok(status) => status,
        Err(_) => return,
    };
    let has_child = jj_status.child_change_id.is_some();
    print!(
        "( %B%F{{magenta}}{}%f%b%F{{244}}{} %f",
        jj_status.change_id.0, jj_status.change_id.1
    );
    if has_child {
        print!("%F{{202}}{}", PARENT_COMMIT)
    } else {
        print!("%F{{green}}{}", LEAF)
    }
    print!("%f ) ")
}
