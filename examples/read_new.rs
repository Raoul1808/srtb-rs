fn main() {
    let srtb = include_str!("../Custom_3.srtb");
    let srtb = spinners::load_srtb_from_str(srtb).unwrap();
    println!("{:#?}", srtb);
}
