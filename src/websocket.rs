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

use std::{
    fs::File,
    io::Write,
    time::Instant
};

use ws::listen;

fn main() {
    println!("WebSocket server is listening on ws://localhost:3000");
    if let Err(error) = listen("localhost:3000", |out: ws::Sender| {
        // The handler needs to take ownership of out, so we use move
        move |msg: ws::Message| {
            println!("Received message");
    
            let start = Instant::now();

            let mut lines = split(msg.as_text().unwrap(), "\n");
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
    
            let mut output_file = File::create("./output.lua")
                .expect("Failed creating output file");

            let output = lines.join("\n");
            output_file.write_all(output.as_bytes())
                .expect("Failed writing to output file");

            std::process::Command::new("stylua")
                .arg("./output.lua")
                .output()
                .expect("Failed formatting output file");

            let formatted_output = std::fs::read_to_string("./output.lua")
                .expect("Failed reading formatted output file");

            out.send(formatted_output)
                .expect("Failed sending formatted output file");

            let duration = start.elapsed();

            println!("Optimized output in {:?}", duration);

            Ok(())
        }
    }) {
        println!("Failed to create WebSocket due to {:?}", error);
    }     
}

