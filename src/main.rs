use std::fs::{create_dir, read_dir, write, File};
use std::io::{stdin, BufRead, BufReader};
use std::collections::{HashMap};
use std::iter::Iterator;

fn main() {
    println!("What folder would you like to filter? (must be relative folder, i.e. './example')");
    let mut input_folder_path = String::new();
    stdin().read_line(&mut input_folder_path).unwrap();
    input_folder_path = input_folder_path.trim().to_string();

    println!("What would you like to name the folder for the newly filtered files? (must be relative folder, i.e. \'./example\')");
    let mut output_folder_path = String::new();
    stdin().read_line(&mut output_folder_path).unwrap();
    output_folder_path = output_folder_path.trim().to_string();

    filter_email_folder(input_folder_path, output_folder_path);
}

fn filter_email_folder(input_folder_path: String, output_folder_path: String) {
    let paths = read_dir(&input_folder_path).unwrap();

    match create_dir(&output_folder_path) {
        Ok(_) => {}
        Err(err) => {
            if err.kind() != std::io::ErrorKind::AlreadyExists {
                panic!("Failed to create filter directory!");
            }
        }
    };

    for path_or in paths {
        let path = path_or.unwrap();
        if path.file_type().unwrap().is_file()
            && path.file_name().into_string().unwrap().ends_with(".txt")
        {
            filter_email_file(
                &input_folder_path,
                &output_folder_path,
                path.file_name().into_string().unwrap(),
            );
        }
    }
}

fn filter_email_file(input_folder_path: &str, output_folder_path: &str, file_name: String) {
    let file = File::open(format!("{}/{}", input_folder_path, &file_name)).unwrap();
    let reader = BufReader::new(&file);
    let second_reader = BufReader::new(&file);

    let mut filtered_emails: Vec<String> = Vec::new();

    let mut valid_domains: HashMap<String, String>  = HashMap::new();


    for line_or in reader.lines() {
        let line_items: Vec<String> = line_or.unwrap().split(",").map(str::to_string).collect();

        //println!("{}", line_items[0]);
        let email: String = format!("{}", line_items[0]);
        let company: String = format!("{}", line_items[1]);
        let system: String = format!("{}", line_items[2]);


        if !email.contains("@byetm") && !email.contains("@sovos") {
            let correct_domain = format!("@{}", email.split("@").last().unwrap()); 
            valid_domains.insert(company, correct_domain.to_string());
        }
        
    }


    for line_or in second_reader.lines() {
        println!("second tab");
        let line: String = line_or.unwrap();
        let email: &str = line.split(",").next().unwrap();
        println!("{}", line);
        if email.contains("@byetm") || email.contains("@sovos") {
            filtered_emails.push(line);
        }
    }
    println!("{:?}", filtered_emails);
    write(
        format!("{}/{}{}", output_folder_path, "filtered_", file_name),
        filtered_emails.join("\r\n"),
    )
    .unwrap();
}
