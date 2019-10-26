use epitech_api::error::EpitechClientError;
use epitech_api::*;
use regex::Regex;
use serde::Deserialize;
use serde_json;

#[derive(Deserialize, Debug)]
struct Course {
    semester: u8,
    scolaryear: u16,
    code: String,
    codeinstance: String,
    title: String,
}

#[derive(Deserialize, Debug)]
struct Module {
    activites: Vec<Activity>,
}

#[derive(Deserialize, Debug)]
struct Activity {
    title: String,
    module_title: String,
    start: String,
    end: String,
    type_code: String,
}

fn usage() -> ! {
    eprintln!("USAGE: epitech-timeline-generator SEMESTER YEAR AUTOLOGIN-LINK");
    std::process::exit(84)
}

fn main() -> Result<(), EpitechClientError> {
    let mut args = std::env::args().skip(1);
    let semester: u8 = args.next().unwrap_or_else(|| {usage()}).parse().unwrap();
    let year: u16 = args.next().unwrap_or_else(|| {usage()}).parse().unwrap();
    let autologin = args.next().unwrap_or_else(|| {usage()});

    // Authenticate
    let client = EpitechClient::builder()
        .autologin(autologin)
        .authenticate()?;
    // Get list of courses (modules)
    let courses = serde_json::from_str::<Vec<Course>>(&client.make_request("/course/filter")?)?;
    // Filter courses
    let courses = courses
        .iter()
        .filter(|course| course.semester == semester && course.scolaryear == year);

    // Get list of projects in each course
    for course in courses {
        let request = format!("/module/{}/{}/{}/", year, course.code, course.codeinstance);
        let module: Module = serde_json::from_str(&client.make_request(request)?)?;
        // Filter projects
        let projects = module
            .activites
            .iter()
            .filter(|activity| activity.type_code == "proj");

        // For all projects, generate the timeline entry
        let date_formatter = Regex::new(r"^(\d+)-(\d+)-(\d+) .+$").unwrap();
        let replacement = "start($3, $2, $1)";
        for project in projects {
            // Format dates correctly
            let start_date = date_formatter.replace(&project.start, replacement);
            let end_date = date_formatter.replace(&project.end, replacement);
            // Create the timeline entry
            println!(
                "['{}', '{}', {}, {}],",
                project.module_title, project.title, start_date, end_date
            );
        }
    }
    Ok(())
}
