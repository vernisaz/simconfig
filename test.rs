extern crate simcfg;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(println!("config={:?}", simcfg::get_config_root().unwrap()))
}
