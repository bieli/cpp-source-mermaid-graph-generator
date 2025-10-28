use cpp_source_mermaid_graph_generator::parser::generate_mermaid_graph;

#[test]
fn test_main_with_nested_braces_and_spacing() {
    let cpp_code = r#"
        int main ( )
        {
            int a = doFunc1();
            if (a == 0)
            {
                exit(42);
            }
            int b = doFunc2();
        }
    "#;
    let expected = r#"
        flowchart TD
        START(test.cpp)
        START --> F001
        F001(CALL doFunc1)
        F001 --> B002{a == 0}
        B002 -->|false| F003
        F003(CALL exit)
        F003 --> F004
        F004(CALL doFunc2)
    "#;

    let actual = generate_mermaid_graph(cpp_code, "test.cpp", &[]);
    assert_eq!(normalize(&actual), normalize(expected));
}

#[test]
fn test_main_with_multiple_returns() {
    let cpp_code = r#"
        main() {
            if (x < 0) {
                return -1;
            }
            if (x == 0) {
                return 0;
            }
            doSomething();
        }
    "#;

    let expected = r#"
        flowchart TD
        START(test.cpp)
        START --> B001{x   0}
        B001 -->|true| RETN1[RETURN -1]
        B001 --> B002{x == 0}
        B002 -->|true| RET0[RETURN 0]
        B002 -->|false| F003
        F003(CALL doSomething)
    "#;

    let actual = generate_mermaid_graph(cpp_code, "test.cpp", &[]);
    assert_eq!(normalize(&actual), normalize(expected));
}

#[test]
fn test_main_with_no_function_calls() {
    let cpp_code = r#"
        main() {
            int x = 5;
            if (x > 0) {
                return 1;
            }
        }
    "#;

    let expected = r#"
        flowchart TD
        START(test.cpp)
        START --> B001{x   0}
        B001 -->|true| RET1[RETURN 1]
    "#;

    let actual = generate_mermaid_graph(cpp_code, "test.cpp", &[]);
    assert_eq!(normalize(&actual), normalize(expected));
}

fn normalize(s: &str) -> String {
    s.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}
