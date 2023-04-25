wit_bindgen::generate!("hermes");

struct Host;
export_hermes!(Host);

impl Hermes for Host {
    fn process(b: Block) -> bool {
        let msg = format!("hello from WASM, slot number is {}", b.slot);
        
        // this function is defined on the host side in Rust code
        host::print(&msg);

        b.slot % 2 == 0
    }
}
