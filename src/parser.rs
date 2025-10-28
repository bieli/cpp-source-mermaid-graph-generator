use std::collections::VecDeque;

pub fn generate_mermaid_graph(input: &str, file_path: &str, skip_list: &[String]) -> String {
    let mut output = String::new();
    output.push_str("flowchart TD\n");
    output.push_str(&format!("    START({})\n", file_path));

    let mut node_id = 1;
    let mut last_node = "START".to_string();
    let mut inside_main = false;
    let mut brace_depth = 0;
    let mut pending_false_branch: Option<String> = None;

    let mut lines: VecDeque<&str> = input.lines().collect();

    while let Some(line) = lines.pop_front() {
        let line = line.trim();

        if !inside_main && line.contains("main") && line.contains("(") && line.contains(")") {
            inside_main = true;
            brace_depth += line.matches('{').count();
            continue;
        }

        if inside_main {
            brace_depth += line.matches('{').count();
            brace_depth -= line.matches('}').count();

            if brace_depth == 0 {
                inside_main = false;
                continue;
            }

            if line.starts_with("if") {
                let raw_condition = extract_condition(line);
                let condition = sanitize(&raw_condition);
                let branch_node = format!("B{:03}", node_id);
                node_id += 1;

                output.push_str(&format!(
                    "    {} --> {}{{{}}}\n",
                    last_node, branch_node, condition
                ));

                if let Some(next_line) = lines.front() {
                    let next_line = next_line.trim();
                    if next_line.starts_with("exit(") {
                        let code = extract_exit_code(next_line).unwrap_or("???".to_string());
                        let exit_node = format!("EXIT{}", code);
                        output.push_str(&format!(
                            "    {} -->|true| {}[EXIT {}]\n",
                            branch_node, exit_node, code
                        ));
                        lines.pop_front();
                    } else if next_line.starts_with("return") {
                        let code = extract_return_code(next_line).unwrap_or("???".to_string());
                        let safe_code = code.replace("-", "N");
                        let return_node = format!("RET{}", safe_code);
                        output.push_str(&format!(
                            "    {} -->|true| {}[RETURN {}]\n",
                            branch_node, return_node, code
                        ));
                        lines.pop_front();
                    }
                }

                pending_false_branch = Some(branch_node.clone());
                last_node = branch_node.clone();
                continue;
            }

            if let Some(func_name) = extract_function_call(line) {
                if skip_list.contains(&func_name) {
                    continue;
                }

                let func_node = format!("F{:03}", node_id);
                if let Some(branch_node) = pending_false_branch.take() {
                    output.push_str(&format!("    {} -->|false| {}\n", branch_node, func_node));
                } else {
                    output.push_str(&format!("    {} --> {}\n", last_node, func_node));
                }

                output.push_str(&format!("    {}(CALL {})\n", func_node, func_name));
                last_node = func_node.clone();
                node_id += 1;
                continue;
            }
        }
    }

    output
}

pub fn extract_function_call(line: &str) -> Option<String> {
    let trimmed = line.trim();
    if trimmed.ends_with(';') && trimmed.contains('(') && trimmed.contains(')') {
        if trimmed.starts_with("if")
            || trimmed.starts_with("switch")
            || trimmed.starts_with("while")
        {
            return None;
        }
        let before_paren = trimmed.split('(').next()?.trim();
        if before_paren.contains('=') {
            let parts: Vec<&str> = before_paren.split('=').collect();
            return Some(parts[1].trim().to_string());
        } else {
            return Some(before_paren.split_whitespace().last()?.to_string());
        }
    }
    None
}

pub fn extract_condition(line: &str) -> String {
    if let Some(start) = line.find('(') {
        if let Some(end) = line[start..].find(')') {
            return line[start + 1..start + end].trim().to_string();
        }
    }
    "condition?".to_string()
}

pub fn extract_exit_code(line: &str) -> Option<String> {
    line.split("exit(")
        .nth(1)?
        .split(')')
        .next()
        .map(|s| s.trim().to_string())
}

pub fn extract_return_code(line: &str) -> Option<String> {
    line.split("return")
        .nth(1)?
        .split(';')
        .next()
        .map(|s| s.trim().to_string())
}

pub fn sanitize(label: &str) -> String {
    label
        .replace(">=", "ge")
        .replace("<=", "le")
        .replace("==", "eq")
        .replace("!=", "ne")
        .replace(">", "gt")
        .replace("<", "lt")
        .replace("[", " ")
        .replace("]", " ")
        .replace("%", "%")
        .replace("\"", " ")
        .replace("|", " ")
        .replace("{", " ")
        .replace("}", " ")
        .replace("(", " ")
        .replace(")", " ")
}
