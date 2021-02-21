use log::{info};

pub mod ip;
pub mod transport;

const WIDTH: usize = 20;

pub trait GettableEndPoints {
    fn get_source(&self) -> String;
    fn get_destination(&self) -> String;
    fn get_payload(&self) -> &[u8];
}

// アプリケーション層ので0田をバイナリで表示する
pub fn print_packet_info(l3: &dyn GettableEndPoints, l4: &dyn GettableEndPoints, protocol: &str) {
    info!("{}", format!(
        "Captured a {} packet from {} | {} to {} | {}\n",
        protocol,
        l3.get_source(),
        l4.get_source(),
        l3.get_destination(),
        l4.get_destination()
    ));

    let payload = l4.get_payload();
    let len = payload.len();

    // ペイロードの表示
    // 指定した定数幅で表示を行う
    for i in 0..len {
        // {:<02X} ← "左詰めで2桁の16進数で表わせ"というフォーマットを指定している
        print!("{:<02X}", payload[i]);
        if i % WIDTH == WIDTH - 1 || i == len - 1 {
            for _ in 0..WIDTH - 1 - (i % WIDTH) {
                print!("{}", " ".repeat(3));
            }
            print!("| ");
            for j in i - i % WIDTH..=i {
                if payload[j].is_ascii_alphabetic() {
                    print!("{}", payload[j] as char);
                } else {
                    // 非ascii文字は.として表示
                    print!(".");
                }
            }
            println!();
        }
    }
    println!("{}", "=".repeat(WIDTH * 3));
    println!();
}
