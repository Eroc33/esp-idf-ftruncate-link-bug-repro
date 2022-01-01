use esp_idf_sys as _;

fn main() {
    esp_idf_sys::link_patches();

    let file = std::fs::File::create("").unwrap();
    file.set_len(0).unwrap();
}
