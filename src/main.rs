use clap::Parser;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::PathBuf;
use std::time::Duration;

/// A simple serial port debugger that reads 23-byte frames and prints them as hex.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The serial port to listen on (e.g., COM3 or /dev/ttyUSB0)
    #[arg(index = 1)]
    port: String,

    /// The baud rate of the serial communication (e.g., 9600)
    #[arg(index = 2)]
    baud_rate: u32,

    /// Optional: The file to write the output to. Data will be appended.
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() -> Result<(), String> {
    // 1. PARSE COMMAND-LINE ARGUMENTS
    let args = Args::parse();

    // Open the output file for appending if one was provided.
    let mut output_file = if let Some(path) = args.output {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .map_err(|e| format!("Failed to open output file '{}': {}", path.display(), e))?;
        println!("Logging output to: {}", path.display());
        Some(file)
    } else {
        None
    };

    println!(
        "Listening on {} at {} baud. Press Ctrl+C to exit.",
        args.port, args.baud_rate
    );

    // 2. CONFIGURE AND OPEN THE SERIAL PORT
    let mut port = serialport::new(&args.port, args.baud_rate)
        .timeout(Duration::from_millis(1000))
        .open()
        .map_err(|e| format!("Failed to open port '{}': {}", args.port, e))?;

    let mut frame_buffer = [0u8; 23];

    // 3. START THE READING LOOP
    loop {
        match port.read_exact(&mut frame_buffer) {
            Ok(_) => {
                let hex_frame: Vec<String> = frame_buffer
                    .iter()
                    .map(|byte| format!("{:02X}", byte))
                    .collect();
                let line = hex_frame.join(" ");

                // Print to console
                println!("{}", line);

                // Write to file if it's open
                if let Some(file) = &mut output_file {
                    if let Err(e) = writeln!(file, "{}", line) {
                        eprintln!("Error writing to file: {}", e);
                    }
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                continue;
            }
            Err(e) => {
                eprintln!("\nA serial port error occurred: {}", e);
                break;
            }
        }
    }

    Ok(())
}