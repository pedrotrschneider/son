use std::fs::File;
use std::io::{self, BufReader, Read};

fn main() -> io::Result<()> {
    let file = File::open("src/test.son")?;
    let mut reader = BufReader::new(file);

    // A buffer to hold the current chunk
    let mut buffer = vec![0u8; 1024];

    // Store leftover bytes from last read that were part of an incomplete UTF-8 character
    let mut leftover = Vec::new();

    loop {
        let n = reader.read(&mut buffer)?;
        if n == 0 {
            break;
        }

        // Combine leftover and newly read data
        leftover.extend_from_slice(&buffer[..n]);

        // Try to decode as much valid UTF-8 from the buffer as possible
        match std::str::from_utf8(&leftover) {
            Ok(valid_str) => {
                print!("{}", valid_str); // All valid
                leftover.clear();
                continue;
            }
            Err(e) => {
                let valid_up_to = e.valid_up_to();

                if valid_up_to > 0 {
                    let valid_str = std::str::from_utf8(&leftover[..valid_up_to]).unwrap();
                    print!("{}", valid_str);
                }

                // Save the trailing bytes (possibly 1 to 3) to try again next iteration
                leftover = leftover[valid_up_to..].to_vec();
            }
        };
    }

    // Print any remaining UTF-8 if valid
    if !leftover.is_empty() {
        if let Ok(valid_str) = std::str::from_utf8(&leftover) {
            print!("{}", valid_str);
        } else {
            eprintln!("Warning: leftover bytes at EOF are not valid UTF-8");
        }
    }

    Ok(())
}
