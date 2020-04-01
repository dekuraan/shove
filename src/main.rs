use std::{env, process::Command};
fn main() {
    let empty_string = "".to_owned();
    let mut commit_message = env::args().collect::<Vec<String>>().join(" ");
    if commit_message == empty_string {
        commit_message =
            "Pushed with ~~love~~ [shove](https://github.com/OriontheCat/shove)".to_owned();
    }
    println!(
        "{:?}",
        Command::new("git")
            .args(&["add", "."])
            .output()
            .expect("failed to `git add .`")
    );
    println!(
        "{:?}",
        Command::new("git")
            .args(&["commit", "-m", &*format!("\"{}\"", commit_message)])
            .output()
            .expect(&*format!("failed to `commit -m {}`", commit_message)),
    );
    println!(
        "{:?}",
        Command::new("git")
            .args(&["push",])
            .output()
            .expect("failed to `git push`"),
    );
}
