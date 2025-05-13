use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    #[arg(long)]
    pub find: Option<String>,

    #[arg(long)]
    pub email: Option<String>,

    #[arg(long)]
    pub status: Option<String>,

    #[arg(long)]
    pub list: bool,

    #[arg(long)]
    pub add_note: Option<String>,

    #[arg(long)]
    pub show_notes: bool,

    #[arg(long)]
    pub clear_notes: bool,

    #[arg(long)]
    pub search_note: Option<String>,

    #[arg(long)]
    pub note_title: Option<String>,

    #[arg(long)]
    pub note_body: Option<String>,

    #[arg(long)]
    pub show_raw_notes: bool,

    #[arg(long)]
    pub show_structured_notes: bool,

    #[arg(long)]
    pub delete_note: Option<u32>,

    #[arg(long)]
    pub edit_note: Option<u32>, // ID to edit

    #[arg(long)]
    pub new_title: Option<String>,

    #[arg(long)]
    pub new_body: Option<String>,

    #[arg(long, value_delimiter = ',')]
    pub tags: Option<Vec<String>>,

    #[arg(long)]
    pub filter_by_tag: Option<String>,

    #[arg(long)]
    pub export_md: bool,

    #[arg(long)]
    pub export_csv: bool,

    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub export_pdf: bool,

    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub open_pdf: bool,
}
