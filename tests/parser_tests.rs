use cpp_source_mermaid_graph_generator::parser::*;

#[test]
fn test_extract_function_call_simple() {
    assert_eq!(
        extract_function_call("doFunc1();"),
        Some("doFunc1".to_string())
    );
}

#[test]
fn test_extract_function_call_with_assignment() {
    assert_eq!(
        extract_function_call("int a = doFunc1();"),
        Some("doFunc1".to_string())
    );
}

#[test]
fn test_extract_condition_basic() {
    assert_eq!(extract_condition("if (a == 0) {"), "a == 0");
}

#[test]
fn test_extract_exit_code() {
    assert_eq!(extract_exit_code("exit(101);"), Some("101".to_string()));
}

#[test]
fn test_extract_return_code() {
    assert_eq!(extract_return_code("return -1;"), Some("-1".to_string()));
}

#[test]
fn test_sanitize_special_chars() {
    let input = "x[k] % 2 < 5";
    let expected = "x k  % 2 lt 5";
    assert_eq!(sanitize(input), expected);
}
