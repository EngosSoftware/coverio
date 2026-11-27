mod cli;
mod input;

#[cfg(target_os = "windows")]
fn win(input: &str) -> String {
    input.replace("coverio ", "coverio.exe ")
}
