use codecrafters_shell::shell::handle_command;

#[test]

fn should_return_command_not_found_message() {
    let result = handle_command("set\n");

    assert_eq!(result, "set: command not found");
}