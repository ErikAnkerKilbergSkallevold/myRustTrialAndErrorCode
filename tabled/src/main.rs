use tabled::{Table, Tabled, Style, Modify, Format, object::Columns};
use colored::*;

fn main() {
#[derive(Tabled)]
struct Language {
    name: &'static str,
    designed_by: &'static str,
    invented_year: usize,
}

let languages = vec![
    Language{
        name: "C",
        designed_by: "Dennis Ritchie",
        invented_year: 1972
    },
    Language{
        name: "Rust",
        designed_by: "Graydon Hoare",
        invented_year: 2010
    },
    Language{
        name: "Go",
        designed_by: "Rob Pike",
        invented_year: 2009
    },
];

let table = Table::new(&languages)
    .with(Style::rounded())
    .with(Modify::new(Columns::single(0)).with(Format::new(|s| s.red().to_string())))
    .with(Modify::new(Columns::single(1)).with(Format::new(|s| s.blue().to_string())))
    .with(Modify::new(Columns::new(2..)).with(Format::new(|s| s.green().to_string())));
println!("{}", table);
}
