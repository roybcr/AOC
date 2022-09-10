use aoc_rs::{hydrothermal_vents as htv, submarine as smar};

fn main() -> std::io::Result<()> {
    smar::submarine();
    println!("Total Dangerous Areas: {}", htv::hydrothermal_vents()?);
    Ok(())
}
