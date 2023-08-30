use fantoccini::{Locator, elements::Element};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub enum Grades {
    G9,
    G10,
    G11,
    G12,
    G9t12,
    G10t12,
    G11t12,
}

#[derive(Serialize, Debug)]
pub enum Category {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

#[derive(Serialize, Debug)]
pub struct Course {
    pub name: String,
    pub is_online: bool,
    pub is_honors: bool,
    pub is_year: bool,
    pub grades: Grades,
    pub category: Category,
    pub abbrev: String,
}

pub async fn get_online(elem: &Element) -> bool {
    let test = elem
        .find_all(Locator::Css(".yearLocationLine span"))
        .await
        .unwrap()[1]
        .text()
        .await
        .unwrap();
    return test == "Online";
}

pub async fn get_honors(elem: &Element) -> bool {
    let test = elem
        .find(Locator::Css("div pcl-course div div div"))
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    return test == "UC Honors";
}

pub async fn get_year(elem: &Element) -> bool {
    let test = elem
        .find(Locator::Css(".yearLocationLine span"))
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    return test == "Full Year,";
}

pub async fn match_grades(elem: &Element) -> Option<Grades> {
    let test = elem
        .find_all(Locator::Css(".gradeLevel span"))
        .await
        .unwrap();

    if test.len() == 1 {
        let grade = test[0].html(true).await.unwrap();
        match grade.as_str() {
            "9" => return Some(Grades::G9),
            "10" => return Some(Grades::G10),
            "11" => return Some(Grades::G11),
            "12" => return Some(Grades::G12),
            &_ => return None,
        };
    } else if test.len() == 2 {
        return Some(Grades::G11t12);
    } else if test.len() == 3 {
        return Some(Grades::G10t12);
    } else if test.len() == 4 {
        return Some(Grades::G9t12);
    } else {
        return None;
    }
}

pub async fn match_category(elem: &Element) -> Option<Category> {
    let test = elem
        .find(Locator::Css(".subject-tag"))
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    return match test.as_str() {
        "A" => Some(Category::A),
        "B" => Some(Category::B),
        "C" => Some(Category::C),
        "D" => Some(Category::D),
        "E" => Some(Category::E),
        "F" => Some(Category::F),
        "G" => Some(Category::G),
        &_ => None,
    };
}


