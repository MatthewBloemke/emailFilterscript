use std::collections::HashMap;
use std::fs::{create_dir, read_dir, write, File};
use std::io::{stdin, BufRead, BufReader};
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

    let mut filtered_emails: HashMap<String, String> = HashMap::new();
    let mut valid_domains: HashMap<String, String> = HashMap::new();
    let mut mapped_systems: HashMap<String, String> = HashMap::new();

    for line_or in reader.lines() {
        let line_items: Vec<String> = line_or.unwrap().split(",").map(str::to_string).collect();

        let email: String = format!("{}", line_items[0]);
        let company: String = format!("{}", line_items[1]);
        let system: String = format!("{}", line_items[2]);

        if !email.contains("@byetm") && !email.contains("@sovos") {
            let correct_domain = format!("@{}", email.split("@").last().unwrap());
            valid_domains.insert(company.clone(), correct_domain.to_string());
            mapped_systems.insert(company, system);
        } else if email.contains("@byetm") || email.contains("@sovos") {
            filtered_emails.insert(email, company);
        }
    }

    write_new_file(
        filtered_emails,
        valid_domains,
        mapped_systems,
        output_folder_path,
        file_name,
    )
}

fn write_new_file(
    filtered_emails: HashMap<String, String>,
    valid_domains: HashMap<String, String>,
    mapped_systems: HashMap<String, String>,
    output_folder_path: &str,
    file_name: String,
) {
    let mut final_filtered_data: Vec<String> = Vec::new();
    for (email, company) in &filtered_emails {
        let system = mapped_systems.get(company).unwrap();
        let correct_domain = valid_domains.get(company).unwrap();
        let corrected_email = format!(
            "{}{}",
            email.split("@").next().unwrap(),
            correct_domain.to_string()
        );
        let final_line_data = format!("{},{},{}", email, system, corrected_email);
        final_filtered_data.push(final_line_data);
    }
    write(
        format!("{}/{}{}", output_folder_path, "filtered_", file_name),
        final_filtered_data.join("\r\n"),
    )
    .unwrap();
}
