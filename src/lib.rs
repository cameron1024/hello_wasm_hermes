wit_bindgen::generate!("hermes");

struct Host;
export_hermes!(Host);

impl Hermes for Host {
    fn process(b: Block) -> bool {
        b.slot % 2 == 0
    }
}
