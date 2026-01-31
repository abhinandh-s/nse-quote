use std::fs;
use std::process::Command;

#[test]
// #[ignore]
fn refresh_groww_fixture() {
    let output = Command::new("sh")
        .arg("-c")
        .arg(
            r#"
            curl -s -H "User-Agent: Mozilla/5.0" \
            "https://www.nseindia.com/api/quote-equity?symbol=GROWW" \
            | jq . 
            "#,
        )
        .output()
        .expect("failed to run curl");

    assert!(output.status.success(), "curl/jq failed");

    fs::write("tests/quote_groww.json", output.stdout).expect("failed to write fixture");
}
