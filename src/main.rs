use clap::Parser;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::PathBuf;
use std::time::Duration;

/// A versatile serial port debugger that reads data frames and prints them to the console and/or files.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The serial port to listen on (e.g., COM3 or /dev/ttyUSB0)
    #[arg(index = 1)]
    port: String,

    /// The baud rate of the serial communication (e.g., 9600)
    #[arg(index = 2)]
    baud_rate: u32,

    /// Set the console output format to hexadecimal instead of the default raw string.
    #[arg(long)]
    hex: bool,

    /// Optional: A generic output file that logs the same format as the console.
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Optional: The file to write hexadecimal output to. Data will be appended.
    #[arg(long)]
    hex_output: Option<PathBuf>,

    /// Optional: The file to write raw, escaped-string output to. Data will be appended.
    #[arg(long)]
    raw_output: Option<PathBuf>,

    /// The frame length in bytes to read. Use 0 for infinite/timeout-based frames.
    #[arg(short, long, default_value_t = 23)]
    length: usize,
}

/// Converts a slice of bytes into a human-readable string, escaping non-printable characters.
fn to_escaped_string(bytes: &[u8]) -> String {
    let mut s = String::new();
    for &byte in bytes {
        // Check for printable ASCII characters (including space)
        if byte >= 0x20 && byte <= 0x7e {
            s.push(byte as char);
        } else {
            // Escape non-printable characters
            s.push_str(&format!("\\x{:02X}", byte));
        }
    }
    s
}

/// Processes a given frame of bytes: prints to console and writes to files as requested.
fn process_frame(
    frame: &[u8],
    args: &Args,
    hex_file: &mut Option<File>,
    raw_file: &mut Option<File>,
    generic_file: &mut Option<File>,
) {
    // Always generate both formats
    let hex_line = frame
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<String>>()
        .join(" ");
    let raw_line = to_escaped_string(frame);

    // Print to console based on the --hex flag
    if args.hex {
        println!("{}", hex_line);
    } else {
        println!("{}", raw_line);
    }

    // Write to specific hex output file if specified
    if let Some(file) = hex_file {
        if let Err(e) = writeln!(file, "{}", hex_line) {
            eprintln!("Error writing to hex file: {}", e);
        }
    }

    // Write to specific raw output file if specified
    if let Some(file) = raw_file {
        if let Err(e) = writeln!(file, "{}", raw_line) {
            eprintln!("Error writing to raw file: {}", e);
        }
    }

    // Write to the generic output file, matching the console format
    if let Some(file) = generic_file {
        let line_to_write = if args.hex { &hex_line } else { &raw_line };
        if let Err(e) = writeln!(file, "{}", line_to_write) {
            eprintln!("Error writing to generic output file: {}", e);
        }
    }
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    // Open output files for appending if provided
    let mut hex_output_file = args.hex_output.as_ref().map(|path| {
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .unwrap_or_else(|e| panic!("Failed to open hex output file '{}': {}", path.display(), e))
    });

    let mut raw_output_file = args.raw_output.as_ref().map(|path| {
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .unwrap_or_else(|e| panic!("Failed to open raw output file '{}': {}", path.display(), e))
    });

    let mut generic_output_file = args.output.as_ref().map(|path| {
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .unwrap_or_else(|e| panic!("Failed to open generic output file '{}': {}", path.display(), e))
    });

    println!(
        "Listening on {} at {} baud. Press Ctrl+C to exit.",
        args.port, args.baud_rate
    );

    let mut port = serialport::new(&args.port, args.baud_rate)
        .timeout(Duration::from_millis(100)) // Shorter timeout for infinite mode responsiveness
        .open()
        .map_err(|e| format!("Failed to open port '{}': {}", args.port, e))?;

    // --- Main Logic ---
    if args.length > 0 {
        // --- FIXED LENGTH MODE ---
        let mut frame_buffer = vec![0; args.length];
        loop {
            // 1. Find the first non-space byte to start the frame
            loop {
                let mut first_byte_buf = [0u8; 1];
                match port.read_exact(&mut first_byte_buf) {
                    Ok(_) => {
                        if first_byte_buf[0] != b' ' { // b' ' is 0x20
                            // Found the start of the frame.
                            frame_buffer[0] = first_byte_buf[0];
                            break; // Exit the "find start" loop
                        }
                        // It was a space, so loop again to read the next byte.
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => continue,
                    Err(e) => {
                        eprintln!("\nA serial port error occurred while waiting for frame: {}", e);
                        return Ok(());
                    }
                }
            }

            // 2. Read the remaining N-1 bytes of the frame
            if args.length > 1 {
                let remaining_bytes = &mut frame_buffer[1..];
                match port.read_exact(remaining_bytes) {
                    Ok(_) => {
                        // Full frame is now in frame_buffer. Process it.
                        process_frame(
                            &frame_buffer,
                            &args,
                            &mut hex_output_file,
                            &mut raw_output_file,
                            &mut generic_output_file,
                        );
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                        eprintln!("\nTimed out while reading frame data. Discarding partial frame.");
                        continue; // Go back to waiting for the next frame
                    }
                    Err(e) => {
                        eprintln!("\nA serial port error occurred while reading frame data: {}", e);
                        break; // Exit the main loop
                    }
                }
            } else {
                // The frame length is just 1, we already have it.
                process_frame(
                    &frame_buffer,
                    &args,
                    &mut hex_output_file,
                    &mut raw_output_file,
                    &mut generic_output_file,
                );
            }
        }
    } else {
        // --- INFINITE/TIMEOUT-BASED MODE ---
        let mut dynamic_buffer: Vec<u8> = Vec::new();
        let mut temp_buf = [0u8; 1024]; // Read in chunks
        loop {
            match port.read(&mut temp_buf) {
                Ok(bytes_read) => {
                    // Add new data to our dynamic buffer
                    dynamic_buffer.extend_from_slice(&temp_buf[..bytes_read]);
                }
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                    // Timeout means the frame has ended. Process it if we have data.

                    // Trim any trailing space characters from the end of the buffer
                    let mut end = dynamic_buffer.len();
                    while end > 0 && dynamic_buffer[end - 1] == b' ' {
                        end -= 1;
                    }
                    let trimmed_frame = &dynamic_buffer[..end];

                    if !trimmed_frame.is_empty() {
                        process_frame(
                            trimmed_frame,
                            &args,
                            &mut hex_output_file,
                            &mut raw_output_file,
                            &mut generic_output_file,
                        );
                    }
                    dynamic_buffer.clear(); // Clear for the next frame
                }
                Err(e) => {
                    eprintln!("\nA serial port error occurred: {}", e);
                    break;
                }
            }
        }
    }

    Ok(())
}
