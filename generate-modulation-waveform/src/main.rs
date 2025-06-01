use eframe::egui;
use num_complex::Complex;
use plotters::coord::Shift;
use plotters::prelude::*;
use rand::Rng;
use serde::Deserialize;
use std::f64::consts::PI;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};

// TOML をマッピングする構造体
#[derive(Deserialize)]
struct Config {
    symbol_count: usize,
    fc: f64,
    fs: f64,
    samples_per_symbol: usize,
    modulation: Modulation,
}

// 変調方式を列挙
#[derive(Deserialize)]
#[serde(rename_all = "UPPERCASE")]
enum Modulation {
    QPSK,
    QAM16,
    QAM64,
}

fn bits_to_levels(bits: &[u8], m: &Modulation) -> (f64, f64) {
    match m {
        Modulation::QPSK => {
            // bits.len() == 2
            let phase = match (bits[0], bits[1]) {
                (0, 0) => PI / 4.0,
                (0, 1) => 3.0 * PI / 4.0,
                (1, 1) => 5.0 * PI / 4.0,
                (1, 0) => 7.0 * PI / 4.0,
                _ => unreachable!(),
            };
            let amp = (2.0f64).sqrt();
            (amp * phase.cos(), amp * phase.sin())
        }
        Modulation::QAM16 => {
            // bits.len() == 4 → I,Q each 2bit
            let map = |b0, u1| match (b0, u1) {
                (0, 0) => -3.0,
                (0, 1) => -1.0,
                (1, 1) => 1.0,
                (1, 0) => 3.0,
                _ => unreachable!(),
            };
            (map(bits[0], bits[1]), map(bits[2], bits[3]))
        }
        Modulation::QAM64 => {
            // bits.len() == 6 → I,Q each 3bit → レベル -7,-5,-3,-1,+1,+3,+5,+7
            let map3 = |b0, b1, b2| {
                // 3bit を Gray などで並べ替えてもよい
                let idx = (b0 << 2 | b1 << 1 | b2) as usize; // 0..7
                // idx=0→-7,1→-5,2→-3,3→-1,4→+1,5→+3,6→+5,7→+7
                -7.0 + idx as f64 * 2.0
            };
            (
                map3(bits[0], bits[1], bits[2]),
                map3(bits[3], bits[4], bits[5]),
            )
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ─── 1. 設定読み込み ─────────────────────────────────
    let toml_str = fs::read_to_string("config.toml")?;
    let cfg: Config = toml::from_str(&toml_str)?;

    // ─── 2. 信号生成 ────────────────────────────────────
    let mut rng = rand::rng();
    let mut time = 0.0;
    let dt = 1.0 / cfg.fs;

    // ビット数／シンボルをモードごとに決定
    let bits_per_symbol = match cfg.modulation {
        Modulation::QPSK => 2,
        Modulation::QAM16 => 4,
        Modulation::QAM64 => 6,
    };

    // 出力ファイル
    let mut writer = std::io::BufWriter::new(fs::File::create("signal.txt")?);

    for _ in 0..cfg.symbol_count {
        // 乱数ビットを準備
        let bits: Vec<u8> = (0..bits_per_symbol)
            .map(|_| rng.random::<bool>() as u8)
            .collect();

        // I/Q レベルを取得
        let (i_level, q_level) = bits_to_levels(&bits, &cfg.modulation);

        // 各サンプルを出力
        for _ in 0..cfg.samples_per_symbol {
            let v = i_level * (2.0 * PI * cfg.fc * time).cos()
                + q_level * (2.0 * PI * cfg.fc * time).sin();
            writeln!(writer, "{:.6} {:.6}", time, v)?;
            time += dt;
        }
    }
    //draw_garph()?;

    demodulate(&cfg)?;
    Ok(())
}

fn draw_garph() -> Result<(), Box<dyn std::error::Error>> {
    // データ読み込み
    let file = File::open("signal.txt")?;
    let reader = BufReader::new(file);

    // 時間と電圧をベクタに格納
    let mut times = Vec::new();
    let mut voltages = Vec::new();

    for line in reader.lines().skip(1) {
        // 最初の行はヘッダー
        let line = line?;
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if parts.len() == 2 {
            let time: f64 = parts[0].parse()?;
            let voltage: f64 = parts[1].parse()?;
            times.push(time);
            voltages.push(voltage);
        }
    }

    // 出力先（PNG画像）
    let root = BitMapBackend::new("signal_plot.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let x_min = *times.first().unwrap_or(&0.0);
    let x_max = *times.last().unwrap_or(&1.0);
    let y_min = voltages.iter().cloned().fold(f64::INFINITY, f64::min);
    let y_max = voltages.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    let mut chart = ChartBuilder::on(&root)
        .caption("Signal Graph", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            times.iter().cloned().zip(voltages.iter().cloned()),
            &RED,
        ))?
        .label("Voltage")
        .legend(|(x, y)| PathElement::new([(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE)
        .draw()?;

    println!("signal_plot.png を生成しました。");

    Ok(())
}

// IQ→ビット列へのマッピング
fn levels_to_bits(i: f64, q: f64, modulation: &Modulation) -> Vec<u8> {
    match modulation {
        Modulation::QPSK => {
            let angle = q.atan2(i);
            // -π〜π を 0〜2π に正規化
            let theta = if angle < 0.0 { angle + 2.0 * PI } else { angle };
            match theta {
                a if (0.0..PI / 2.0).contains(&a) => vec![1, 0], // 第4象限 → 10
                a if (PI / 2.0..PI).contains(&a) => vec![0, 0],  // 第1象限 → 00
                a if (PI..3.0 * PI / 2.0).contains(&a) => vec![0, 1], // 第2象限 → 01
                _ => vec![1, 1],                                 // 第3象限 → 11
            }
        }
        Modulation::QAM16 => {
            // IとQそれぞれでレベルを4段階に量子化
            let quantize = |v: f64| {
                if v < -2.0 {
                    vec![0, 0]
                } else if v < 0.0 {
                    vec![0, 1]
                } else if v < 2.0 {
                    vec![1, 1]
                } else {
                    vec![1, 0]
                }
            };
            let mut bits = quantize(i);
            bits.extend(quantize(q));
            bits
        }
        Modulation::QAM64 => {
            // IとQそれぞれでレベルを8段階に量子化
            let quantize = |v: f64| {
                let levels = [-7.0, -5.0, -3.0, -1.0, 1.0, 3.0, 5.0, 7.0];
                let mut closest = 0;
                let mut min_dist = f64::MAX;
                for (i, &lvl) in levels.iter().enumerate() {
                    let dist = (v - lvl).abs();
                    if dist < min_dist {
                        min_dist = dist;
                        closest = i;
                    }
                }
                // インデックスを3bitに変換
                vec![
                    ((closest >> 2) & 1) as u8,
                    ((closest >> 1) & 1) as u8,
                    (closest & 1) as u8,
                ]
            };
            let mut bits = quantize(i);
            bits.extend(quantize(q));
            bits
        }
    }
}

fn demodulate(config: &Config) -> std::io::Result<()> {
    let file = File::open("signal.txt")?;
    let reader = BufReader::new(file);

    let mut samples = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if let Some((_t_str, v_str)) = line.split_once(' ') {
            let v: f64 = v_str.parse().unwrap_or(0.0);
            samples.push(v);
        }
    }

    let mut recovered_bits = Vec::new();
    let spb = config.samples_per_symbol;
    let fc = config.fc;
    let fs = config.fs;

    for i in 0..config.symbol_count {
        let mut i_sum = 0.0;
        let mut q_sum = 0.0;
        for j in 0..spb {
            let idx = i * spb + j;
            if idx >= samples.len() {
                break;
            }

            let t = idx as f64 / fs;
            let s = samples[idx];
            i_sum += s * (2.0 * PI * fc * t).cos();
            q_sum += s * (2.0 * PI * fc * t).sin();
        }

        let i_avg = i_sum / spb as f64;
        let q_avg = q_sum / spb as f64;

        let bits = levels_to_bits(i_avg, q_avg, &config.modulation);
        recovered_bits.extend(bits);
    }

    println!("Recovered bits: {:?}", recovered_bits);
    Ok(())
}
