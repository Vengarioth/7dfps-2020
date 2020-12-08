use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
pub struct Opt {
    /// Activate debug mode
    // short and long flags (-d, --debug) will be deduced from the field's name
    #[structopt(short, long)]
    debug: Option<bool>,
}

impl Opt {
    pub fn new () -> Self {
        Opt::from_args()
    }
    pub fn is_debug(&self) -> bool
    {
        match &self.debug {
            Some(debug_opt) => {
                match debug_opt {
                    true => { return true; },
                    false => {return false; },
                }
            }
            None => {
                return false;
            }
        }
    }
}