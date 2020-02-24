mod rcat {
    use std::fs::write;
    use std::process::Command;
    use tempfile::tempdir;

    static HAIKU: &'static str = "Rust delights the soul
Fresh rains of chromatic bliss
On a setting sun";

    #[test]
    fn calling_with_non_existent_file() {
        let output = Command::new("./target/debug/rcat")
            .arg("foobar")
            .output()
            .expect("failed to execute process");

        assert_eq!(
            String::from_utf8_lossy(&output.stderr),
            "foobar: No such file or directory\n"
        );
    }

    #[test]
    fn calling_with_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("calling_with_file");
        write(&file_path, HAIKU.as_bytes()).unwrap();

        let output = Command::new("./target/debug/rcat")
            .arg(file_path.into_os_string())
            .output()
            .expect("failed to execute process");

        assert_eq!(String::from_utf8_lossy(&output.stdout), HAIKU);
    }

    #[test]
    fn calling_with_number_options() {
        let numbered_haiku = "     1\tRust delights the soul
     2\tFresh rains of chromatic bliss
     3\tOn a setting sun";
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("calling_with_file");
        write(&file_path, HAIKU.as_bytes()).unwrap();

        let output = Command::new("./target/debug/rcat")
            .arg("-n")
            .arg(file_path.into_os_string())
            .output()
            .expect("failed to execute process");

        assert_eq!(String::from_utf8_lossy(&output.stdout), numbered_haiku);
    }
}
