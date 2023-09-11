use std::{process::Command, env, path::Path};

fn main() {

    if env::var_os("BUILD_UI").is_some() {
        println!("Building UI");
        Command::new("npm")
            .args(&["ci"])
            .current_dir("./src/ui")
            .output()
            .unwrap();

        Command::new("npm")
            .args(&["run", "build"])
            .current_dir("./src/ui")
            .output()
            .unwrap();

        let build_type = env::var_os("PROFILE").unwrap();
        let ui_out_path = Path::new("target").join(build_type).join("ui");

        println!("Copying UI to {}", ui_out_path.as_os_str().to_str().unwrap());

        if cfg!(target_os = "Windows") {
            Command::new("Copy-Item")
                .args(&[r#".\src\ui\dist\"#, ui_out_path.to_str().unwrap(), "-Recurse"])
                .output()
                .unwrap();
        } else {
            Command::new("cp")
                .args(&["-r", "./src/ui/dist", ui_out_path.to_str().unwrap()])
                .output()
                .unwrap();
        }
    }

    // cargo_emit::rerun_if_changed!(
    //     "build.rs",
    //     "src/ui/"
    // );
}
