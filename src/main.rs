use std::{
    // cmp::Eq,
    // hash::{Hash},
    fs::File,
    io::{Read, Write},
};

mod utils;
use utils::{
    split,
    rename_services,
    rename_requires,
    rename_find_childs,
    rename_dot_variables,
    rename_new_variables,
    rename_new_tables,
};


fn main() {
    let mut input_file = File::open("./input.lua")
        .expect("Failed opening input.lua, make sure it exists");

    let mut input = String::new();
    input_file.read_to_string(&mut input)
        .expect("Failed reading input to string");

    let input = input.as_str();

    let mut lines = split(input, "\n");
    lines = lines.iter().map(|s| s.trim().to_string()).collect();


    lines = rename_services(&mut lines)
        .expect("Failed renaming services");

    lines = rename_requires(&mut lines)
        .expect("Failed renaming requires");

    lines = rename_find_childs(&mut lines)
        .expect("Failed renaming find childs");

    lines = rename_dot_variables(&mut lines)
        .expect("Failed renaming variables");

    lines = rename_new_variables(&mut lines)
        .expect("Failed renaming new variables");

    lines = rename_new_tables(&mut lines)
        .expect("Failed renaming new tables");


    // write output to  output.lua
    let mut output_file = File::create("./output.lua")
        .expect("Failed creating output file");

    let output = lines.join("\n");
    output_file.write_all(output.as_bytes())
        .expect("Failed writing to output file");


    std::process::Command::new("stylua")
        .arg("./output.lua")
        .output()
        .expect("Failed running stylua command");

    let formatted_output = std::fs::read_to_string("./output.lua")
        .expect("Failed reading formatted output file");

    println!("{}", formatted_output);
}
