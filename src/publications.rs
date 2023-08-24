use std::path::PathBuf;

use super::repr::Publication;
use super::html::{Break, Span};
use build_html::*;
use nom::{sequence::delimited, bytes::complete::{tag, take_until}, combinator::recognize};
// TODO: Sort publictions somehow
impl Publication {
    fn to_table_cell(self) -> TableCell {
        let pid = self.id;
        let auth_len = self.authors.len();
        let formatted_authors = self
            .authors
            .into_iter()
            .enumerate()
            .map(|(i, auth)| {
                if i == auth_len - 1 {
                    format!("and {}", &auth)
                } else {
                    auth
                }
            })
            .collect::<Vec<_>>()
            .join(", ");
        let abstract_html = markdown::to_html(&self.abstr.unwrap());
        let pid_abstr = format!("{pid}-abstr");
        let formatted_venue = format!("{} {}", self.venue, self.year);
        let mut base_cell = TableCell::new(TableCellType::Data)
            .with_html(Span::new(self.title, "paper-title", None::<String>))
            .with_html(Span::new(formatted_venue, "venue", None::<String>))
            .with_html(Break {})
            .with_html(Span::new(formatted_authors, "authors", None::<String>))
            .with_html(Break {})
            .with_link_attr(
                "#",
                "Show Abstract",
                [(
                    "onclick",
                    &format!("toggleHide(\'{}\'); return false;", &pid_abstr)[..],
                )],
            )
            .with_html(Break {})
            .with_html(Span::new(abstract_html, "abstract", Some(pid_abstr)));
        if let Some(link) = self.arxiv {
            let _ = base_cell.add_link(link, "Arxiv");
        }
        if let Some(link) = self.doi {
            let _ = base_cell.add_link(link, "DOI");
        }
        if let Some(link) = self.code {
            let _ = base_cell.add_link(link, "Code");
        }
        
        if let Some(link) = self.datasets {
            let _ = base_cell.add_link(link, "Datasets");
        }
        base_cell
    }
}

fn publications_to_html(pubs: Vec<Publication>) -> Container {
    let tbl_html = pubs
        .into_iter()
        .map(|publ| TableRow::new().with_cell(publ.to_table_cell()))
        .fold(Table::new(), |tbl, r| tbl.with_custom_body_row(r));
    Container::default()
        .with_attributes([("class", "publications")])
        .with_header(2, "Publications")
        .with_table(tbl_html)
}

fn parse_publication_file(pub_file: PathBuf) -> Publication { 
    let raw_pub_text = std::fs::read_to_string(pub_file).expect("Could not read publication file.");
    let yaml_delimeter = tag::<_, _, ()>("---"); 
    let (abstr_md_text, opts_yaml_text) = delimited(
        &yaml_delimeter,
        recognize(take_until("---")), 
        &yaml_delimeter)(&raw_pub_text[..]).unwrap();
    let mut publ : Publication = serde_yaml::from_str(opts_yaml_text).unwrap();
    publ.abstr = Some(abstr_md_text.to_string());
    publ
}

pub fn compile_publications(pubs_dir: PathBuf) -> Container { 
    let mut pubs = std::fs::read_dir(pubs_dir)
        .expect("Cannot read pubs_dir")
        .map(|f| parse_publication_file(f.unwrap().path()))
        .collect::<Vec<_>>();
    pubs.sort_by(|p1, p2| p2.order.cmp(&p1.order));
    publications_to_html(pubs)
}
