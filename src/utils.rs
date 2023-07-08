use std::collections::HashMap;
use anyhow::Result;
use regex::Regex;

pub fn split(input: &str, to_split: &str) -> Vec<String> {
    let res: Vec<String> = input.split(to_split).map(|s| s.to_string()).collect();
    return res;
}

fn count_chars(string: &str, character: char) -> usize {
    return string.chars().filter(|&c| c == character).count()
}

fn is_number(input: &str) -> bool {
    match input.parse::<f64>() {
        Ok(_) => true,  // The string is a valid number
        Err(_) => false,  // The string is not a valid number
    }
}

fn string_has(string: &str, must_contain: &[&str], must_have: &[&str], cant_contain: &[&str]) -> bool {
    let words: Vec<&str> = string.split_whitespace().collect();
    
    for &word in must_contain {
        if !words.iter().any(|&w| w == word) {
            return false;
        }
    }

    for &word in must_have {
        if string.contains(word) == false {
            return false;
        }
    }
    
    for &word in cant_contain {
        if words.iter().any(|&w| w == word) {
            return false;
        }
    }
    
    return true;
}

pub fn replace_variables(lines: &[String], original_name: String, new_name: String) -> Vec<String> {
    let mut result_lines: Vec<String> = Vec::new();
    let split_chars: [&str; 8] = [" ", ".", ":", "(", ")", "[", "]", ","];

    for line in lines {
        let mut string_stack = String::new();
        let mut words: Vec<String> = Vec::new();

        for char in line.chars() {
            if split_chars.contains(&char.to_string().as_str()) {

                words.push(string_stack.clone());
                words.push(char.to_string());

                string_stack = String::new();
            } else {
                string_stack.push(char);
            }
        }

        words.push(string_stack.clone());

        let mut new_words: Vec<String> = Vec::new();
        
        for word in words {
            if word == original_name {
                new_words.push(new_name.to_string());
            } else {
                new_words.push(word.to_string());
            }
        }

        result_lines.push(new_words.join(""));
    }

    return result_lines;

}
 
pub fn rename_services(lines: &[String]) -> Result<Vec<String>> {
    let mut service_values: HashMap<String, String> = HashMap::new();
    let mut result_lines: Vec<String> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        result_lines.push(line.clone());

        if string_has(line, &["="], &["game:GetService"], &[]) == false {
            continue
        }

        let service_name = &split(line, "\"")[1];
        let var_name = &split(line, " ")[1];

        result_lines[i] = format!("local {} = game:GetService(\"{}\");", service_name, service_name);

        service_values.insert(var_name.to_string(), service_name.to_string());
    }

    for (original_name, new_name) in &service_values {
        result_lines = replace_variables(&result_lines, original_name.to_string(), new_name.to_string());
    }

    Ok(result_lines)
}

pub fn rename_requires(lines: &[String]) -> Result<Vec<String>> {
    let mut require_values: HashMap<String, String> = HashMap::new();
    let mut result_lines: Vec<String> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        result_lines.push(line.clone());


        if string_has(line, &["=", "local"], &["require"], &[]) == false {
            continue
        }

        if count_chars(line, '(') != 1 || count_chars(line, ')') != 1 {
            continue
        }

        // local v_u_551 = require(v_u_2.Modules.ProjectileHandler)
        let var_name = &split(line, " ")[1];

        let require_path = &split(line, "require(")[1];
        let require_path = &require_path[..require_path.len() - 1];

        let new_name = &split(require_path, ".");
        let new_name = &new_name[new_name.len() - 1];
        

        result_lines[i] = format!("local {} = require({});", new_name, require_path);
        require_values.insert(var_name.to_string(), new_name.to_string());
    }

    for (original_name, new_name) in &require_values {
        result_lines = replace_variables(&result_lines, original_name.to_string(), new_name.to_string());
    }

    Ok(result_lines)

}

pub fn rename_find_childs(lines: &[String]) -> Result<Vec<String>> {
    let mut child_values: HashMap<String, String> = HashMap::new();
    let mut result_lines: Vec<String> = Vec::new();
    let mut variables_renamed: HashMap<String, i32> = HashMap::new();

    for (i, line) in lines.iter().enumerate() {
        result_lines.push(line.clone());


        if !line.contains("WaitForChild") && !line.contains("FindFirstChild") {
            continue
        }

        if string_has(line, &["=", "local"], &[], &["or", "and", ".."]) == false {
            continue
        }

        if count_chars(line, ':') > 1  || count_chars(line, '"') != 2 { 
            continue
        }

        if count_chars(line, '(') != 1 || count_chars(line, ')') != 1 {
            continue
        }

        let original_name = &split(line, " ")[1];

        let func_type = &split(line, ":")[1];
        let func_type = &split(func_type, "(")[0];

        let parent_string = &split(line, ":")[0];

        if &split(parent_string, "= ").len() == &(1 as usize) {
            continue
        }

        let parent_string = &split(parent_string, "= ")[1];

        let delimiter = format!("{}(\"", func_type);

        let child_string = &split(line, &delimiter)[1].to_string();
        let child_string = &child_string[..child_string.len() - 2];


        // let mut new_name: &str = &original_name;

        let mut new_name: String = child_string.to_string().clone();
        if variables_renamed.get(&new_name) == None {
            variables_renamed.insert(new_name.to_string(), 0);
        }

        let amount = variables_renamed.get(&new_name).unwrap();
        if amount > &(0 as i32) {
            new_name = format!("{}_{}", child_string, amount);
        }

        variables_renamed.insert(new_name.to_string(), amount + 1);

        // println!("{} -> {}", child_string, new_name);

        result_lines[i] = format!("local {} = {}:{}(\"{}\")", original_name, parent_string, func_type, new_name);
        child_values.insert(original_name.to_string(), new_name.to_string());
    }

    for (original_name, new_name) in &child_values {
        result_lines = replace_variables(&result_lines, original_name.to_string(), new_name.to_string());
    }

    Ok(result_lines)  

}

pub fn rename_dot_variables(lines: &[String]) -> Result<Vec<String>> {
    let mut variable_values: HashMap<String, String> = HashMap::new();
    let mut result_lines: Vec<String> = Vec::new();
    let mut variables_renamed: HashMap<String, i32> = HashMap::new();

    let re = Regex::new(r"^[A-Za-z0-9_ .]*=[A-Za-z0-9_ .]*$").unwrap();

    for (i, line) in lines.iter().enumerate() {
        result_lines.push(line.clone());

        if string_has(line, &["=", "local"], &[], &["or", "and"]) == false {
            continue
        }

        if re.is_match(line) == false {
            continue
        }

        let original_name = &split(line, " ")[1];

        if original_name == "_" {
            continue
        }

        let var_value = &split(line, "= ")[1];
        let var_value = &mut split(var_value, ".");

        if var_value.len() < 2 || is_number(var_value.join("").as_str()) {
            continue
        }

        let mut new_name = var_value[var_value.len() - 2..].join("_");

        if variables_renamed.get(&new_name) == None {
            variables_renamed.insert(new_name.to_string(), 0);
        }

        let amount = variables_renamed.get(&new_name).unwrap();
        if amount > &(0 as i32) {
            new_name = format!("{}_{}", new_name, amount);
        }

        variables_renamed.insert(new_name.to_string(), amount + 1);

        // if amount_renamed > 0 {
        //     new_name = format!("{}_{}", new_name, amount_renamed);
        // }

        // amount_renamed += 1;

        variable_values.insert(original_name.to_string(), new_name.to_string());
        result_lines[i] = format!("local {} = {}", new_name, var_value.join("."));
    }


    for (original_name, new_name) in &variable_values {
        result_lines = replace_variables(&result_lines, original_name.to_string(), new_name.to_string());
    }

    Ok(result_lines)  

}

pub fn rename_new_variables(lines: &[String]) -> Result<Vec<String>> {
    let mut variable_values: HashMap<String, String> = HashMap::new();
    let mut result_lines: Vec<String> = Vec::new();
    let mut variables_renamed: HashMap<String, i32> = HashMap::new();

    for (i, line) in lines.iter().enumerate() {
        result_lines.push(line.clone());

        if string_has(line, &["=", "local"], &[".new"], &[":"]) == false {
            continue
        }
        if count_chars(line, '.') < 1 {
            continue
        }

        let original_name = &split(line, " ")[1];
        let right_side = &split(line, "= ")[1];

        let newvar_type = &split(line, "= ")[1];
        let mut newvar_type = split(newvar_type, ".new")[0].clone().to_string();
        
        if variables_renamed.get(&newvar_type) == None {
            variables_renamed.insert(newvar_type.to_string(), 0);
        }

        let amount = variables_renamed.get(&newvar_type).unwrap();
        if amount > &(0 as i32) {
            newvar_type = format!("{}_{}", newvar_type, amount);
        }

        variables_renamed.insert(newvar_type.to_string(), amount + 1);

        variable_values.insert(original_name.to_string(), newvar_type.to_string());
        result_lines[i] = format!("local {} = {}", newvar_type, right_side);
    }


    for (original_name, new_name) in &variable_values {
        result_lines = replace_variables(&result_lines, original_name.to_string(), new_name.to_string());
    }

    Ok(result_lines)  

}

pub fn rename_new_tables(lines: &[String]) -> Result<Vec<String>> {
    let mut table_values: HashMap<String, String> = HashMap::new();
    let mut result_lines: Vec<String> = Vec::new();
    let mut amount_renamed = 1;

    for (i, line) in lines.iter().enumerate() {
        result_lines.push(line.clone());

        if string_has(line, &["=", "local", "{}"], &[], &[":", ","]) == false {
            continue
        }
        if count_chars(line, '{') > 1 || count_chars(line, '}') > 1 {
            continue
        }

        let original_name = &split(line, " ")[1];
        let right_side = &split(line, "= ")[1];

        if right_side != "{}" {
            continue
        }

        let new_name = format!("table_{}", amount_renamed);

        amount_renamed += 1;

        table_values.insert(original_name.to_string(), new_name.to_string());
        result_lines[i] = format!("local {} = {{}}", new_name);
    }


    for (original_name, new_name) in &table_values {
        result_lines = replace_variables(&result_lines, original_name.to_string(), new_name.to_string());
    }

    Ok(result_lines)  

}
