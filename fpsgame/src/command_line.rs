use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
pub struct Opt {
    /// Activate debug mode
    // short and long flags (-d, --debug) will be deduced from the field's name
    #[structopt(short, long)]
    debug: bool,

    #[structopt(short, long)]
    windowed: bool,
}

impl Opt {
    pub fn new () -> Self {
        let opt = Opt::from_args();
        println!("{:?}", opt);
        opt
    }
    pub fn is_debug(&self) -> bool
    {
        self.debug
    }
    pub fn is_windowed(&self) -> bool
    {
        self.windowed
    }
}