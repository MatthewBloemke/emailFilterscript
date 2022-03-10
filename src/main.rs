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

    match filter_email_folder(input_folder_path, output_folder_path) {
        Ok(_) => {},
        Err(err) => {
            println!("Program failed: {}", err);
        }
    };
    println!("Hit enter to Close");
    let mut close = String::new();
    stdin().read_line(&mut close).unwrap();
}

fn filter_email_folder(input_folder_path: String, output_folder_path: String) -> Result<(), String> {
    let paths = match read_dir(&input_folder_path) {
        Ok(paths) => paths,
        Err(err) => return Err(format!("Failed to open input directory: {}", err))
    };

    match create_dir(&output_folder_path) {
        Ok(_) => {}
        Err(err) => {
            if err.kind() != std::io::ErrorKind::AlreadyExists {
                panic!("Failed to create filter directory!");
            }
        }
    };

    for path_or in paths {
        let path = match path_or {
            Ok(path) => path,
            Err(err) => return Err(format!("Failed to extract path: {}", err))
        };
        let is_file = match path.file_type() {
            Ok(file_type) => file_type.is_file(),
            Err(err) => return Err(format!("Unable to check file type: {}", err))
        };

        let file_name = match path.file_name().into_string() {
            Ok(file_name) => file_name,
            Err(_) => return Err(format!("Unable to check file name"))
        };

        if is_file
            && file_name.ends_with(".txt")
        {
            filter_email_file(
                &input_folder_path,
                &output_folder_path,
                file_name,
            )?;
        }
    }

    Ok(())
}

fn filter_email_file(input_folder_path: &str, output_folder_path: &str, file_name: String) -> Result<(), String> {

    let file = match File::open(format!("{}/{}", input_folder_path, &file_name)) {
        Ok(file) => file,
        Err(err) => return Err(format!("Could not open file: {}", err))
    };
    let reader = BufReader::new(&file);

    let mut filtered_emails: HashMap<String, String> = HashMap::new();
    let mut valid_domains: HashMap<String, String> = HashMap::new();
    let mut mapped_systems: HashMap<String, String> = HashMap::new();

    for line_or in reader.lines() {
        let line_items: Vec<String> = match line_or {
            Ok(line) => line.split(",").map(str::to_string).collect(),
            Err(err) => return Err(format!("Could not get line data: {}", err))
        };
        if line_items.len() >= 3 {
            let email: String = line_items[0].clone();
            let company: String = line_items[1].clone();
            let system: String = line_items[2].clone();
            
            if email.to_lowercase().contains("@byetm") || email.to_lowercase().contains("@sovos") {
                filtered_emails.insert(email, company.clone());
                mapped_systems.insert(company, system);
            } else {
                let correct_domain = format!("@{}", match email.split("@").last() {
                    Some(last) => last,
                    None => ""
                });
                valid_domains.insert(company.clone(), correct_domain.to_string());
                mapped_systems.insert(company, system);
            }            
        }



    }

    write_new_file(
        filtered_emails,
        valid_domains,
        mapped_systems,
        output_folder_path,
        file_name,
    )?;

    Ok(())
}

fn write_new_file(
    filtered_emails: HashMap<String, String>,
    valid_domains: HashMap<String, String>,
    mapped_systems: HashMap<String, String>,
    output_folder_path: &str,
    file_name: String,
) -> Result<(), String> {
    println!("{:?}", &valid_domains);
    let mut final_filtered_data: Vec<String> = Vec::new();
    for (email, company) in &filtered_emails {
        let system = match mapped_systems.get(company) {
            Some(system) => system,
            None => return Err(format!("System not found for company: {}", company))
        };
        let correct_domain = match valid_domains.get(company) {
            Some(domain) => domain,
            None => return Err(format!("Domain not found! Company: {}, System: {}", company, system))
        };
        let corrected_email = format!(
            "{}{}",
            match email.split("@").next() {
                Some(email) => email,
                None => ""
            },
            correct_domain.to_string()
        );
        let final_line_data = format!("{},{},{}", email, system, corrected_email);
        final_filtered_data.push(final_line_data);
    }
    match write(
        format!("{}/{}{}", output_folder_path, "filtered_", file_name),
        final_filtered_data.join("\r\n"),
    ) {
        Ok(_) => {},
        Err(err) => return Err(format!("Could not create file: {}", err))
    };

    Ok(())
}
