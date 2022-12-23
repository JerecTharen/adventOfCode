mod utils;
mod tests;
use utils::hello::hello::respond_hello as respond_hello;

fn main() {
    println!("Hello There!");
    respond_hello();

    //Read each line
    //Get all the visible trees from a line as coordinates
    //Create lines for each possible direction
    //Get all the unique coordinates
}
