use fastembedserver::embed;

fn main() {
    println!("{}", embed("").unwrap().len())
}