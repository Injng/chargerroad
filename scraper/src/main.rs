use std::fs::File;
use std::io::Write;
use fantoccini::{ClientBuilder, Locator};
use async_io::Timer;
use std::time::Duration;
use serde_json;
use coursescraper::*; 

#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    let client = ClientBuilder::native().connect("http://localhost:4444").await.expect("failed to connect");
    let mut course_list: Vec<Course> = Vec::new();
    client.goto("https://hs-articulation.ucop.edu/agcourselist/institution/2266").await.expect("failed to access webpage");
    Timer::after(Duration::from_secs(5)).await;

    for i in client.find_all(Locator::Css(".grid-row")).await.expect("not found").iter() {
        i.click().await.unwrap();
    }
    
    for i in client.find_all(Locator::Css(".expand-in")).await.expect("not found").iter() {
        let curr_course = Course {
            name: i.find(Locator::Css("h5")).await.unwrap().html(true).await.unwrap(),
            is_online: get_online(i).await,
            is_honors: get_honors(i).await,
            is_year: get_year(i).await,
            grades: match_grades(i).await.unwrap(),
            category: match_category(i).await.unwrap(),
            abbrev: i.find(Locator::Css(".one-col")).await.unwrap().html(true).await.unwrap(),
        };
        course_list.push(curr_course);
    }

    let course_json = serde_json::to_string(&course_list)?;
    let mut output = File::create("courses.json")?;
    write!(output, "{}", course_json)?;
    Ok(())
}
