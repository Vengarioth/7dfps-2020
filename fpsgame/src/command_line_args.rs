use std::*;

pub struct CommandLineArgs{
    pub args: Vec<String>
}

impl CommandLineArgs {
    pub fn has_arg(&self, argument: String) -> bool
    {
        for arg in &self.args
        {
            if *arg == argument
            {
                return true;
            }
        }
        false
    }
}
