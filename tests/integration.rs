use assert_cmd::Command;
use std::fs;

#[test]
fn test_grayscale_ascii_generation() {
    let output_file = "ascii_art.png";
    if fs::metadata(output_file).is_ok() {
        fs::remove_file(output_file).unwrap();
    }

    Command::cargo_bin("ascii_image")
        .unwrap()
        .args(&["-i", "assets/test_img_bw.jpeg", "-w", "80"])
        .assert()
        .success();

    assert!(fs::metadata(output_file).is_ok(), "ascii_art.png was not generated");
}

#[test]
fn test_color_ascii_generation() {
    let output_file = "ascii_art.png";
    if fs::metadata(output_file).is_ok() {
        fs::remove_file(output_file).unwrap();
    }

    Command::cargo_bin("ascii_image")
        .unwrap()
        .args(&["-i", "assets/test_img_color.jpeg", "-w", "80", "--color"])
        .assert()
        .success();

    assert!(fs::metadata(output_file).is_ok());
}
