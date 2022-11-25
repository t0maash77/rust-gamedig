
use gamedig::games::mc;

fn main() {
    let response = mc::query("localhost", None); //or Some(25565), None is the default protocol port (which is 25565)
    match response {
        Err(error) => println!("Couldn't query, error: {error}"),
        Ok(r) => println!("{:#?}", r)
    }
}
