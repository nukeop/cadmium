use clap::Parser;

// Transforms the colors of a given image according to a given palette
#[derive(Parser, Debug)]
#[clap(author="nukeop", version="1.0.0", name="cadmium", long_about = None)]
pub struct CadmiumArgs {
    // image to modify
    #[clap(short, long, value_parser)]
    pub input: Option<String>,
    // palette to apply
    #[clap(short, long, value_parser)]
    pub palette: Option<String>,
    // output file
    #[clap(short, long, value_parser)]
    pub output: Option<String>,
}

pub fn read_cl_args() -> CadmiumArgs {
    CadmiumArgs::parse()
}
