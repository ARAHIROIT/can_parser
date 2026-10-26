use std::fs::File;
use std::io::{BufRead, BufReader};
use serde::Deserialize;

// --- 実務意識ポイント①：定数の定義（マジックナンバーの排除） ---
const SPEED_LIMIT: f32 = 100.0;               // 速度超過の基準（km/h）
const SHIFT_MASK_BITS: i32 = 0x03;            // シフト位置を特定するためのビットマスク
const REVERSE_SHIFT_BIT: i32 = 1;             // R（リバース）レンジを表すビット値
const BRAKE_ON_BIT: i32 = 0x01;               // ブレーキONを表すビットマスク
const SLEEP_DURATION_MS: u64 = 1000;          // リアルタイム監視の間隔（ミリ秒）

/// CAN通信のパケット構造を定義する設定構造体
#[derive(Deserialize, Debug)]
struct CanConfig {
    speed_index: usize,
    speed_gain: f32,
    brake_index: usize,
    shift_bits_index: usize,
}

fn main() {
    println!("=== リアルタイムCAN通信解析システム (実務対応・面接アピール版) ===");
    println!("ネットワーク「vcan0」の疑似監視を開始します...");

    // --- 実務意識ポイント②：丁寧なエラーハンドリング ---
    let config_file = File::open("config.json")
        .expect("[致命的エラー] 設定ファイル(config.json)が開けませんでした。配置を確認してください。");
    let config_reader = BufReader::new(config_file);
    let config: CanConfig = serde_json::from_reader(config_reader)
        .expect("[致命的エラー] JSONのパース（解析）に失敗しました。フォーマットを確認してください。");

    let log_file = File::open("can_log.txt")
        .expect("[致命的エラー] 疑似入力データ(can_log.txt)が開けませんでした。");
    let log_reader = BufReader::new(log_file);

    // ログファイルを1行ずつリアルタイムに解析（ストリーム処理の模倣）
    for (index, line) in log_reader.lines().enumerate() {
        let line_number = index + 1; // エラー表示用の行番号
        let line_string = match line {
            Ok(v) => v,
            Err(e) => {
                eprintln!("[警告] {}行目の読み込みに失敗したためスキップします: {}", line_number, e);
                continue;
            }
        };

        // 空白行は安全にスキップ
        if line_string.trim().is_empty() {
            continue;
        }

        // カンマ区切りのパケットデータを分解し、前後の空白をトリム
        let packet: Vec<&str> = line_string.split(',').map(|s| s.trim()).collect();
        
        // データの個数が仕様を満たしていない場合は、クラッシュさせずにスキップ
        if packet.len() < 3 {
            eprintln!("[警告] {}行目のデータ個数が不足しているため無視します。", line_number);
            continue;
        }

        println!("\n---【パケットリアルタイム受信（レコード: {}）】---", line_number);

        // 1. 車速の計算と出力
        let row_speed: i32 = packet[config.speed_index].parse().unwrap_or(0);
        let speed_kmh = row_speed as f32 + config.speed_gain; 
        println!("現在の車速: {:.0} km/h", speed_kmh);

        // 2. ブレーキ状態の判定（ビット演算）
        let brake_bit: i32 = packet[config.brake_index].parse().unwrap_or(0);
        let is_brake_on = brake_bit & BRAKE_ON_BIT == 1;
        println!("ブレーキ状態: {}", if is_brake_on { "ON (制動中)" } else { "OFF" });

        // 3. シフト位置の判定（マッチングガード）
        let shift_bits: i32 = packet[config.shift_bits_index].parse().unwrap_or(0);
        let shift_position = match shift_bits & SHIFT_MASK_BITS {
            0 => "P (パーキング)",
            1 => "R (リバース)",
            2 => "N (ニュートラル)",
            3 => "D (ドライブ)",
            _ => "不明（予期せぬ値）",
        };
        println!("現在のシフト: {}", shift_position);

        // 4. 車両の危険状態を判定するビジネスロジック
        if speed_kmh > SPEED_LIMIT {
            // 100km/hを超えてバック（R）ギアが入り、かつブレーキを踏んでいるケース（緊急回避を想定）
            if (shift_bits & SHIFT_MASK_BITS) == REVERSE_SHIFT_BIT && is_brake_on {
                println!("[情報] 危険回避アラート：高速域でのRレンジ切り替えを検知。ブレーキ作動中につき安全です。");
            } else {
                println!("[警告] 速度超過：安全基準（{}km/h）を超えています！減速してください。", SPEED_LIMIT);
            }
        }

        // 車両ECUからの通信間隔を再現するためにスリープを入れる
        std::thread::sleep(std::time::Duration::from_millis(SLEEP_DURATION_MS));
    }
}
