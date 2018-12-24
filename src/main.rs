use std::fs::File;
use std::io::Read;

fn cstackify() -> Result<(), String> {
    let mut args = std::env::args();
    let program_name = args.next().unwrap_or_else(|| "cstackify".to_string());

    if let Some(filename) = args.next() {
        let identifier = args.next().unwrap_or_else(|| "data".to_string());
        let file = File::open(&filename)
            .map_err(|_| format!("Could not open the specified file (`{}`).", filename))?;
        let len = file.metadata()
            .map_err(|_| format!("Could not read the metadata of the specified file (`{}`).",
                                 filename))?
            .len();

        println!("long {}_len = 0x{:x};", identifier, len);
        println!("unsigned char {0}[{0}_len];", identifier);

        for (index, byte) in file.bytes().enumerate() {
            let byte = byte.map_err(|_| format!("Failed to read byte #{} of the specified file.",
                                                index))?;
            println!("{}[0x{:x}] = 0x{:02x};", identifier, index, byte);
        }

        Ok(())
    } else {
        Err(format!("Generates 32-bit C code to load a file on the stack.
Usage: {} <FILE> [IDENTIFIER]", program_name))
    }
}

fn main() {
    match cstackify() {
        Ok(()) => (),
        Err(message) => {
            println!("{}", message);
            std::process::exit(1);
        },
    }
}
