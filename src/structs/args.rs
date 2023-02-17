pub use clap::Parser;

/// A simple program to test apis
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
   
   /// The base url of the request
   #[arg(short, long, required = true)]
   url: String,

   /// if -g is passed, the request will be a GET request
    #[arg(long, default_value_t = false)]
    get: bool,

    /// if -p is passed, the request will be a POST request
    #[arg(long, default_value_t = false)]
    post: bool,

    /// if -d is passed, the request will be a DELETE request
    #[arg(long, default_value_t = false )]
    delete: bool,

    /// if -u is passed, the request will be a PUT request
    #[arg(long, default_value_t = false)]
    put: bool,
}

impl Args {
    pub fn is_validate_request_type(&self) -> bool {
        let mut count = 0;
        if self.get {
            count += 1;
        }
        if self.post {
            count += 1;
        }
        if self.delete {
            count += 1;
        }
        if self.put {
            count += 1;
        }
        if count > 1 || count == 0 {
            return false;
        }
        true
    }
}