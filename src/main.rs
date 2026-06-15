use colored::*;
use std::env;
use std::fs;
use serde::Deserialize;
use suprint_rs::suprint_return;


//Todo list :
//support transparent pixels
//make code cleaner
//make possible to import any compatible json file with a cli argument and add an argument to dismiss the warning specified on line 13
//make a tiny pixel art editor to stop having to place all the pixels manually
//display warning when specified heigth isn't equal to the true heigth
//fix bug : when a heigth is specified (and is odd), if there is another line in the file, it is displayed where it shouldn't be
//handle "thread 'main' panicked at src/main.rs:28:26:  index out of bounds: the len is 17 but the index is 17  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace   when specified heigth isn't equal to real one"
//throw an error when terminal size is inferior to the picture's one

//Done :
//make sure it works with impair values
//handle errors when there are different line width

// Importing the object that will be used to display the image
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Image {
    width: u32,
    heigth: u32,
    image: Vec<Vec<[u8; 3]>>,
}

fn check(image: &Vec<Vec<[u8; 3]>>, heigth: u32, width: u32) -> bool {
    let mut line_wrong_width = vec![];
    for i in 0..heigth {
        let line = &image[i as usize];
        if line.len() != (width as usize) {
            line_wrong_width.push(i + 1);
        }
    } 
    if !line_wrong_width.is_empty() {
        let mut lines_str = String::from("");
        for i in 0..line_wrong_width.len() {
            if (i + 1) == line_wrong_width.len() {
                lines_str.push_str(&format!("{}", line_wrong_width[i]));
            } else {
                lines_str.push_str(&format!("{}, ", line_wrong_width[i]));
            }
        }
        let message = format!("The following lines width aren't equal to the one specified in the width field of the provided file : {}. (Note : We are talking about the line numbers of your array, not of your file).", lines_str);
        let text_lines = suprint_return(message);
        for i in 0..text_lines.len() { // rendering the colored text line by line
            let usize_i: usize = i as usize;
            println!("{}", text_lines[usize_i].truecolor(183, 65, 14));
        }
        return false;
    }
    true
}

fn is_odd(n: u32) -> bool {
    n % 2 != 0
}

fn display(parsed_file: Image) {
    let image; //change name
    if is_odd(parsed_file.heigth) == true {
        image = (0..(parsed_file.heigth -1)).step_by(2);
    } else {
        image = (0..parsed_file.heigth).step_by(2);
    }
     
    /*if is_odd(parsed_file.heigth) == true {
        image = (0..(parsed_file.heigth - 1)).step_by(2)
    } else {
        
    }*/

    if parsed_file.heigth < parsed_file.image.len().try_into().unwrap() {
        let text_lines = suprint_return(format!("The number of lines in your image ({}) is greater than the specified image width ({}). The lines below line {} won't be displayed.", parsed_file.image.len(), parsed_file.heigth, parsed_file.heigth));
        for i in 0..text_lines.len() {
            let usize_i: usize = i as usize;
            println!("{}", text_lines[usize_i].truecolor(183, 65, 14));
        }
    } else if parsed_file.heigth > parsed_file.image.len().try_into().unwrap() {
        let text_lines = suprint_return(format!("The number of lines in your image ({}) lowest than the specified image width ({}). Some lines are", parsed_file.image.len(), parsed_file.heigth));
        for i in 0..text_lines.len() {
            let usize_i: usize = i as usize;
            println!("{}", text_lines[usize_i].truecolor(200, 0, 0));
        }
    }
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

    if is_odd(parsed_file.heigth) == true {
        let last_index = parsed_file.heigth - 1;
        let line_last = &parsed_file.image[last_index as usize];
        for j in 0..line_last.len() {
            let pixel = &line_last[j as usize];
            print!(
                "{}",
                "▀".truecolor(pixel[0], pixel[1], pixel[2])
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
    
    let width_check = check(&parsed_file.image, parsed_file.heigth, parsed_file.width);

    if width_check == true {
        display(parsed_file);
    } else {
        println!("Operation aborted. Please fix the specified line width(s) and retry.");
    }

}