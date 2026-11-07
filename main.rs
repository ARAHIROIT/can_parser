use std::fs::File;
use std::io::{BufRead, BufReader};
use serde::Deserialize;
// 💡 【修正の核心①】コンパイラの指示通り、data() を有効化するための「EmbeddedFrame」を明示的にインポートします
use socketcan::{CanSocket, Socket, CanFrame, EmbeddedFrame}; 

// =========================================================================
// 🏢 定数定義（ロードマップの安全設計・フェールセーフ仕様）
// =========================================================================
const SPEED_LIMIT: f32 = 100.0;               // 速度超過を検知するための安全基準しきい値（100.0 km/h）
const SHIFT_MASK_BITS: i32 = 0x03;            // 下位2ビット（シフト位置情報）を切り出すためのANDマスク値
const REVERSE_SHIFT_BIT: i32 = 1;             // マスク後の値が「1」の時にR（リバース）レンジであると特定する基準値
const BRAKE_ON_BIT: i32 = 0x01;               // 最下位ビット（ブレーキON/OFF）を抽出するためのANDマスク値（0x01）
const SLEEP_DURATION_MS: u64 = 1000;          // 🟨【フェーズ2】1秒ごとにトントン自動パースするための周期タイマー（1000ms）

// =========================================================================
// 💡 🟢【フェーズ1】config.json の設定（仕様書）を格納するための箱
// =========================================================================
#[derive(Deserialize, Debug)]
struct CanConfig {
    speed_index: usize,       // 配列0 (先頭): 車速データの場所
    speed_gain: f32,          // 車速の足し算に使う補正値
    brake_index: usize,       // 配列1 (真ん中): ブレーキ状態の場所
    shift_bits_index: usize,  // 配列2 (一番右): シフトポジションの場所
}

// =========================================================================
// 💡 🟢【フェーズ1】車両の状態を一括で保持・管理（カプセル化）するフラグ構造体
// =========================================================================
struct VehicleStatus {
    speed_kmh: f32,               // 補正後の最終的な車速（km/h）
    is_brake_on: bool,            // ブレーキがONかどうかの真偽値
    shift_position: &'static str, // 解析された現在のシフト位置の文字列
    is_speed_over: bool,           // 安全基準（100km/h）を超えているかどうかの判定フラグ
}

fn main() {
    println!("=== リアルタイム通信解析システム (最上級・全フェーズ融合ハイブリッド完全版) ===");
    println!("処理モード: 📊 早期例外ガードロジック ＆ ロードマップ完全準拠仕様");

    // -------------------------------------------------------------------------
    // 📂 1. 🟢【フェーズ1】仕様書ファイル（config.json）の動的読み込み
    // -------------------------------------------------------------------------
    let config_file = File::open("config.json")
        .expect("[致命的エラー] 設定ファイル(config.json)が開けませんでした。");
    let config_reader = BufReader::new(config_file);
    let config: CanConfig = serde_json::from_reader(config_reader)
        .expect("[致命的エラー] JSONのパース（解析）に失敗しました。");

    // =========================================================================
    // 🎛️ 🟦【最上級プラン】モード切り替えスイッチ
    // =========================================================================
    let use_real_can = true; 

    if use_real_can {
        // =========================================================================
        // 🌐 🟦【最上級プラン】本物の SocketCAN 待ち受け無限ループ処理
        // =========================================================================
        let socket = CanSocket::open("vcan0")
            .expect("[エラー] Linuxの vcan0 回線が開けませんでした。OrbStackを確認してください。");

        println!("【最上級モード】Linux回線「vcan0」に接続。生のパケット電気信号を常時待ち受け中...");

        loop {
            if let Ok(frame) = socket.read_frame() {
                // 🔒 【修正の核心②】EmbeddedFrame トレイトが読み込まれたため、data() が100%安全に動作します！
                let packet: &[u8] = match &frame {
                    CanFrame::Data(data_frame) => data_frame.data(),
                    _ => continue, 
                };

                if packet.len() < 3 { continue; }

                // 🚗 🟢【フェーズ1】配列0 (先頭): 車速データ（生の数値に補正値を「足し算」して km/h に変換）
                let row_speed: i32 = packet[config.speed_index] as i32;
                let speed_kmh = row_speed as f32 + config.speed_gain; 

                // 🛑 🟢【フェーズ1】配列1 (真ん中): ブレーキ状態（0x01 のビットマスクで ON/OFF を判定）
                let brake_bit: i32 = packet[config.brake_index] as i32;
                let is_brake_on = brake_bit & BRAKE_ON_BIT == 1;

                // ⚙️ 🟢【フェーズ1】配列2 (一番右): シフトポジション（下位2ビットを切り出し、match 構文で型安全に条件分岐）
                let shift_bits: i32 = packet[config.shift_bits_index] as i32;
                let shift_position = match shift_bits & SHIFT_MASK_BITS {
                    0 => "P (パーキング)",
                    1 => "R (リバース)",       
                    2 => "N (ニュートラル)",
                    3 => "D (ドライブ)",
                    _ => "未定義（不正データ）", 
                };

                // 📊 共通の解析・診断関数へ流し込み
                analyze_and_print(speed_kmh, is_brake_on, shift_position, shift_bits, 0);
            }
        }
    } else {
        // =========================================================================
        // 📄 🟨【フェーズ2】一括で取り込んだログファイルを1行ずつ安全に解析（完全保存版）
        // =========================================================================
        let log_file = File::open("can_log.txt")
            .expect("[致命的エラー] can_log.txt が開けませんでした。");
        let log_reader = BufReader::new(log_file);

        println!("【ファイルモード】can_log.txt のデータを1秒周期で自動パース中...");

        for (index, line) in log_reader.lines().enumerate() {
            let line_number = index + 1; 
            
            let line_string = match line {
                Ok(v) => v,
                Err(_) => continue,
            };

            if line_string.trim().is_empty() { continue; }

            let packet: Vec<&str> = line_string.split(',').map(|s| s.trim()).collect();
            if packet.len() < 3 { continue; }

            let row_speed: i32 = packet[config.speed_index].parse().unwrap_or(0);
            let speed_kmh = row_speed as f32 + config.speed_gain; 

            let brake_bit: i32 = packet[config.brake_index].parse().unwrap_or(0);
            let is_brake_on = brake_bit & BRAKE_ON_BIT == 1;

            let shift_bits: i32 = packet[config.shift_bits_index].parse().unwrap_or(0);
            let shift_position = match shift_bits & SHIFT_MASK_BITS {
                0 => "P (パーキング)",
                1 => "R (リバース)",       
                2 => "N (ニュートラル)",
                3 => "D (ドライブ)",
                _ => "未定義（不正データ）",
            };

            analyze_and_print(speed_kmh, is_brake_on, shift_position, shift_bits, line_number);

            std::thread::sleep(std::time::Duration::from_millis(SLEEP_DURATION_MS));
        }
    }
}

// =========================================================================
// 🚨 🟢【フェーズ1】異常系先行ガード処理（高速域でのRレンジ誤進入を最優先でキャッチ）
// =========================================================================
fn analyze_and_print(speed_kmh: f32, is_brake_on: bool, shift_position: &'static str, shift_bits: i32, line_number: usize) {
    let status = VehicleStatus {
        speed_kmh,
        is_brake_on,
        shift_position,
        is_speed_over: speed_kmh > SPEED_LIMIT,
    };

    if line_number > 0 {
        println!("\n---【パケットファイル受信（レコード: {}）】---", line_number);
    } else {
        println!("\n---【本物CANパケット受信検知】---");
    }

    println!("📈 現在の車速: {:.0} km/h", status.speed_kmh);
    println!("🛑 ブレーキ状態: {}", if status.is_brake_on { "ON (制動中)" } else { "OFF" });
    println!("⚙️ 現在のシフト: {}", status.shift_position);

    if status.is_speed_over {
        if (shift_bits & SHIFT_MASK_BITS) == REVERSE_SHIFT_BIT && status.is_brake_on {
            println!("[情報] 危険回避アラート：高速域でのRレンジ切り替えを検知。ブレーキ作動中につき安全です。");
        } else {
            println!("[警告] 速度超過（安全基準: {}km/h超）！減速してください。", SPEED_LIMIT);
        }
    } else {
        println!("[STATUS] 車両状態は正常です。");
    }
}
