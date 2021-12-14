use serial2::SerialPort;
use telegraf::{Client, Metric};

#[derive(Metric)]
struct CarbonDioxide {
    field1: f32,
    #[telegraf(tag)]
    tag1: String,
}

fn main() {
    println!("Hello, world!");

    let mut client = Client::new("tcp://localhost:8094").unwrap();
    let point = CarbonDioxide {
        field1: 1800.234,
        tag1: "afo1".to_string(),
    };
    client.write(&point).unwrap();
    let port = SerialPort::open("/dev/ttyACM1", 115200).unwrap();
    let mut buffer = [0; 256];
    loop {
        if let Ok(read) = port.read(&mut buffer) {
            let string = std::str::from_utf8(&buffer[..read]).unwrap();
            if string.contains("ppm") {
                if let Some(number) = string.split(' ').next() {
                    if let Ok(number) = number.parse::<f32>() {
                        let point = CarbonDioxide {
                            field1: number,
                            tag1: "afo1".to_string(),
                        };
                        client.write(&point).unwrap();
                        println!("wtf {}", number);
                    }
                }
            }
        }
    }
}
