use clap::Parser;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::PathBuf;
use std::time::Duration;

/// A versatile serial port debugger that reads data frames and prints them to the console and/or files.
/// It handles both newline-terminated frames and frames defined by a timeout.
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
}

/// Converts a slice of bytes into a human-readable string, escaping non-printable characters.
fn to_escaped_string(bytes: &[u8]) -> String {
    let mut s = String::new();
    for &byte in bytes {
        // Use Rust's built-in escape_default for a robust representation
        for char_part in (byte as char).escape_default() {
            s.push(char_part);
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
    // Do not process empty frames.
    if frame.is_empty() {
        return;
    }

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
        // A short timeout is crucial for the hybrid approach.
        .timeout(Duration::from_millis(100))
        .open()
        .map_err(|e| format!("Failed to open port '{}': {}", args.port, e))?;

    // --- Main Logic: Hybrid Line-Buffering and Timeout Mode ---
    let mut port_buffer: Vec<u8> = Vec::new();
    let mut temp_buf = [0; 1024]; // Read in chunks for efficiency

    loop {
        match port.read(&mut temp_buf) {
            Ok(bytes_read) => {
                // Add newly read data to our main buffer
                port_buffer.extend_from_slice(&temp_buf[..bytes_read]);

                // Process all complete lines found in the buffer
                while let Some(newline_pos) = port_buffer.iter().position(|&b| b == b'\n') {
                    // Drain the buffer up to and including the newline to get the line.
                    let line_bytes = port_buffer.drain(..=newline_pos).collect::<Vec<u8>>();

                    // Trim off trailing '\n' and '\r' to get the actual frame content.
                    let end = line_bytes.as_slice()
                        .iter()
                        .rposition(|&b| b != b'\n' && b != b'\r')
                        .map_or(0, |i| i + 1);

                    process_frame(
                        &line_bytes[..end],
                        &args,
                        &mut hex_output_file,
                        &mut raw_output_file,
                        &mut generic_output_file,
                    );
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                // A timeout occurred. This can mean two things:
                // 1. No data is being sent (the buffer is empty).
                // 2. A frame was sent that was NOT terminated by a newline.

                // If the buffer has data, process it as a complete frame.
                if !port_buffer.is_empty() {
                    // We can simply process the entire buffer content as one frame.
                    // Then clear it to be ready for the next one.
                    let frame_to_process = port_buffer.clone();
                    port_buffer.clear();

                    process_frame(
                        &frame_to_process,
                        &args,
                        &mut hex_output_file,
                        &mut raw_output_file,
                        &mut generic_output_file,
                    );
                }
                // If the buffer was empty, do nothing and just continue waiting.
                continue;
            }
            Err(e) => {
                // A more serious error occurred (e.g., port disconnected).
                eprintln!("\nA serial port error occurred: {}", e);
                break; // Exit the main loop
            }
        }
    }

    Ok(())
}
