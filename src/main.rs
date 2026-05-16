use colored::*;
use std::env;
use std::fs;
use serde::Deserialize;


//Todo list :
//make sure it works with impair values
//handle errors when there are different line width -> partially handled but it would be better if all lines with wrong width are displayed at once
//support transparent pixels
//make code cleaner
//make possible to import any compatible json file with a cli argument
//make a tiny pixel art editor to stop having to place all the pixels manually


// Importing the object that will be used to display the image
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Image {
    width: u32,
    height: u32,
    image: Vec<Vec<[u8; 3]>>,
}

fn check(image: &Vec<Vec<[u8; 3]>>, height: u32, width: u32) -> bool {
    for i in 0..height {
        let line = &image[i as usize];
        if line.len() != (width as usize) {
            let message = format!("Line {} isn't equal to the specified line width.", i + 1);
            println!(
                "{}",
                message.truecolor(183, 65, 14)
            );
            return false;
        }
    } 
    true
}

fn display(parsed_file: Image) {
    let image = (0..parsed_file.height).step_by(2); //change that 
    for i in image {
        let line = &parsed_file.image[i as usize];
        let second_line = &parsed_file.image[(i + 1) as usize];
        for j in 0..line.len() {
            let pixel_one = &line[j as usize];
            let pixel_two = &second_line[j as usize];
            print!(
                "{}",
                "▀".truecolor(pixel_one[0], pixel_one[1], pixel_one[2]).on_truecolor(pixel_two[0], pixel_two[1], pixel_two[2])
            );
        }
        print!("\n");  
    }
}

fn main() {
    let current_dir = env::current_dir().unwrap();
    let file_path = current_dir.join("src/smiley.json");
    let file = fs::read_to_string(file_path).expect("Error while reading the file");
    let parsed_file: Image = serde_json::from_str(&file).unwrap();
    
    let width_check = check(&parsed_file.image, parsed_file.height, parsed_file.width);

    if width_check == true {
        display(parsed_file);
    } else {
        println!("Operation aborted. Please fix the specified line width and retry.");
    }

}