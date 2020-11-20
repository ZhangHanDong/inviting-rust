
mod environment;
mod error;
mod conf;

fn main() {

    let conf = conf::PoemConfig::read_config();
    println!("{:#?}", conf);

}