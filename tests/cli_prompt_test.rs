use encfs_cracker::cli_utils::{MockPasswordPrompt, PasswordPrompt};
use std::cell::RefCell;

#[test]
fn test_mock_password_prompt() {
    let mock = MockPasswordPrompt {
        responses: RefCell::new(vec!["pass1".to_string(), "pass2".to_string()]),
    };

    assert_eq!(mock.prompt("p1: ").unwrap(), "pass1");
    assert_eq!(mock.prompt("p2: ").unwrap(), "pass2");
    assert!(mock.prompt("p3: ").is_err());
}
