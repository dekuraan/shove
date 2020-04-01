use std::{
    env,
    process::{Command, Output},
    str,
};
fn main() {
    let empty_string = "".to_owned();
    let mut commit_message = env::args().collect::<Vec<String>>().join(" ");
    if commit_message == empty_string {
        commit_message =
            "Pushed with ~~love~~ [shove](https://github.com/OriontheCat/shove)".to_owned();
    }
    let output = Command::new("git")
        .args(&["add", "."])
        .output()
        .expect("failed to `git add .`");
    print_output(output);
    let output = Command::new("git")
        .args(&["commit", "-m", &*format!("\"{}\"", commit_message)])
        .output()
        .expect(&*format!("failed to `commit -m {}`", commit_message));
    print_output(output);
    let output = Command::new("git")
        .args(&["push"])
        .output()
        .expect("failed to `git push`");
    print_output(output);
}

fn print_output(output: Output) {
    let Output {
        stdout,
        stderr,
        status,
    } = output;
    println!("{}", str::from_utf8(&stdout).expect("stdout malformed"));
    eprintln!("{}", str::from_utf8(&stderr).expect("stderr malformed"));
}
