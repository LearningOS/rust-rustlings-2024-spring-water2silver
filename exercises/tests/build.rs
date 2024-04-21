//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.
#[allow(unused_variables)]
fn main() {
    // In tests7, we should set up an environment variable
    // called `TEST_FOO`. Print in the standard output to let
    // Cargo do it.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs(); // What's the use of this timestamp here?
    // "{}{}","rustc-env=TEST_FOO=".to_string(),timestamp
    // "Your command here with {}{}, please checkout exercises/tests/build.rs",
    let your_command = format!(
        "{}{}",
        "rustc-env=TEST_FOO=".to_string(),
        timestamp
    );
    println!("cargo:{}", your_command);

    // In tests8, we should enable "pass" feature to make the
    // testcase return early. Fill in the command to tell
    // Cargo about that.
    let your_command = "rustc-cfg=feature=\"pass\"";
    println!("cargo:{}", your_command);
}
