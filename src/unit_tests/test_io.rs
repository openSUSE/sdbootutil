use super::super::io::*;

#[test]
fn test_non_existent_command() {
    let result = get_command_output("command_that_does_not_exist", &["arg1"]);

    assert!(
        result.is_err(),
        "Expected an error when executing a non-existent command"
    );
}

#[test]
fn test_command_output() {
    let command_output = get_command_output("echo", &["This is a test"]).unwrap();
    assert_eq!(
        command_output, "This is a test",
        "Expected 'This is a test' as command output"
    );
}
