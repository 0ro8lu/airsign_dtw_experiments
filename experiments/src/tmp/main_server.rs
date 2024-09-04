use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::net::{SocketAddr, UdpSocket};

fn main() -> std::io::Result<()> {
    let file_path = std::env::args().nth(1).expect("no file_path given");

    let addr = "0.0.0.0:8888";
    let socket = UdpSocket::bind(addr)?;
    let mut signature_counter: u32 = 0;

    let mut visualization_file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(format!("{}/visualization_file", file_path))?;

    let mut buf = [0u8; 100000];

    loop {
        let (num_bytes, src) = socket.recv_from(&mut buf)?;
        let message = String::from_utf8_lossy(&buf[..num_bytes]);

        match message.as_ref() {
            "HFQ!" => {
                println!("Got a SYN signal, sending ACK back");
                send_acknowledgment(&socket, &src)?;
            }
            "NS" => {
                write_next_signature(&mut visualization_file, signature_counter)?;
                signature_counter += 1;
            }
            _ => {
                // Create .csv file
                unpack_data_csv(&file_path, message.as_ref(), &signature_counter)?;

                // Create json file for visualization
                unpack_data_custom(&mut visualization_file, message.as_ref())?;
            }
        }
    }
}

fn write_next_signature(
    visualization_file: &mut File,
    signature_counter: u32,
) -> std::io::Result<()> {
    writeln!(visualization_file, "{}", signature_counter)?;
    Ok(())
}

fn unpack_data_custom(visualization_file: &mut File, data: &str) -> std::io::Result<()> {
    let mut final_string = String::new();

    for point in data.split(';') {
        let values: Vec<f32> = point
            .split(',')
            .filter_map(|s| s.parse::<f32>().ok())
            .collect();

        if values.len() != 0 {
            final_string.push_str(&format!("{},{},{};", values[0], values[1], values[2]));
        }
    }

    writeln!(visualization_file, "{}", final_string)?;

    Ok(())
}

fn unpack_data_csv(file_path: &str, data: &str, signature_counter: &u32) -> std::io::Result<()> {
    let mut f = File::options()
        .write(true)
        .append(true)
        .create(true)
        .open(format!("{}/signature_{}.csv", file_path, signature_counter))?;
    for point in data.split(';') {
        // save this to a newline of a .csv file
        writeln!(&mut f, "{}", point)?;
    }

    Ok(())
}

fn send_acknowledgment(socket: &UdpSocket, src: &SocketAddr) -> std::io::Result<()> {
    let response = b"ACK";
    socket.send_to(response, src)?;
    Ok(())
}
