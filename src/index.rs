use build_html::*;

fn build_header() -> HtmlPage {
    HtmlPage::new()
        .with_meta([
            ("http-equiv", "Content-Type"),
            ("content", "text/html; charset=utf-8")
        ])
        .with_meta([
            ("http-equiv", "Content-Style-Type"),
            ("content", "text/css")
        ])
        .with_meta([
            ("name", "author"),
            ("content", "John Gouwar"),
        ])
        .with_meta([
            ("name", "keywords"),
            ("content", "John Gouwar, John Gouwar Northeastern, John Gouwar programming languages"),
        ])
        .with_meta([
            ("name", "viewport"),
            ("content", "width=device-width, initial-scale=1.0")
        ])
        .with_head_link("../assets/style.css", "stylesheet")
        .with_script_link(".../assets/utils.js")
        .with_title("John Gouwar | Northeastern PRL")
        .with_header_attr(1, "John Gouwar", [("class", "page-title")])
        .with_head_link_attr(
            3, 
            "PhD. Student in Northeastern University's Programming Research Laboratory",
            [("class", "person-title")])
        .with_image_attr("../assets/headshot.png", "Photo of John Gouwar", [("class", "prof-pic")])
}

fn build_footer() -> Container { 
    Container::new(ContainerType::Footer)
        .with_link(
            "https://github.com/JohnGouwar", 
            Container::default()
                .with_attributes([("class", "gh-logo")])
                .with_image("../assets/github-mark.png", "Github logo")
                .to_html_string()
        )
        .with_link(
            "https://scholar.google.com/citations?user=WQbxx8wAAAAJ&hl",
            Container::default()
                .with_attributes([("class", "gs-logo")])
                .with_image("../assets/google-scholar.png", "Google Scholar logo")
                .to_html_string()
        )
}


pub fn build_index(sections: Vec<Container>) -> String { 
    let mut base = build_header();
    for sec in sections { 
        base.add_container(sec);
    }
    base.with_container(build_footer()).to_html_string()
}