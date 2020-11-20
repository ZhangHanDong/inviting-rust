
use toml_demo;

fn main() {

    let conf = toml_demo::PoemConfig::read_config();
    println!("{:#?}", conf);

}
