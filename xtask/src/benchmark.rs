
        let output = std::process::Command::new("cargo")
            .arg("bench")
            .arg("benchmark")
            .arg("-p")
            .arg("tree-sitter-cli")
            .arg("--no-run")
            .arg("--message-format=json")
            .spawn()?
            .wait_with_output()?;

        
}
