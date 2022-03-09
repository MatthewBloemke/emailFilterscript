use std::fs::{create_dir, read_dir, write, File};
use std::io::{stdin, BufRead, BufReader};

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
    let file = File::open(format!("{}/{}", input_folder_path, file_name)).unwrap();
    let reader = BufReader::new(file);

    let mut filtered_emails: Vec<String> = Vec::new();

    for line_or in reader.lines() {
        let line: String = line_or.unwrap();
        let email: &str = line.split(",").next().unwrap();

        if email.contains("@byetm") || email.contains("@sovos") {
            filtered_emails.push(line);
        }
    }

    write(
        format!("{}/{}{}", output_folder_path, "filtered_", file_name),
        filtered_emails.join("\r\n"),
    )
    .unwrap();
}
