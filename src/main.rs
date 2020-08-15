mod errors;
mod register;

fn main() {
    match register::register_bmc() {
        Ok(()) => println!("Registered!"),
        Err(e) => println!("Unable to register: {:?}", e),
    };
}