use exfinalfusion::exfinalfusion;
fn main() {
    let embeddings = exfinalfusion::from_file("../../similarity.fifu").unwrap();
    let result = exfinalfusion::get_embeddings(embeddings, "Berlin");
    println!("{:?}", &result);
}
