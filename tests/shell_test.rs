use codecrafters_shell::shell::handle_command;
use codecrafters_shell::shell::handle_input;
#[test]
fn should_return_command_not_found_message() {
    let result = handle_command("set\n");

    assert_eq!(result, "set: command not found");
}

#[test]
fn prints_command_not_found_for_input() {
    let input = "hello\n";
    let output = handle_input(input);

    assert_eq!(output, "hello: command not found");
}