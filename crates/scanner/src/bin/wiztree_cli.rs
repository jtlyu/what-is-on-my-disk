//! CLI tool to test WizTree integration and compare with walkdir.
//!
//! Usage:
//!   cargo run -p diskwise-scanner --bin wiztree_cli                     # WizTree C 盘扫描
//!   cargo run -p diskwise-scanner --bin wiztree_cli --walkdir <PATH>   # + walkdir 对比

use std::io::{Read, Write};
use std::path::PathBuf;
use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let walkdir_path = args
        .windows(2)
        .find(|w| w[0] == "--walkdir")
        .map(|w| PathBuf::from(&w[1]));

    // ── WizTree 扫描 C: 盘 ──────────────────────────────────
    let wiztree = match diskwise_scanner::wiztree::find_wiztree_executable() {
        Some(p) => {
            println!("[WizTree] 找到: {}", p.display());
            p
        }
        None => {
            eprintln!("❌ 未找到 WizTree64.exe");
            eprintln!("   请确认已安装 WizTree (https://diskanalyzer.com)");
            std::process::exit(1);
        }
    };

    let output_dir = std::env::temp_dir().join("wiztree_test");
    std::fs::create_dir_all(&output_dir).expect("create temp dir");
    let csv_path = output_dir.join(format!(
        "wiztree_C_{}.csv",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    ));

    print!("[WizTree] 扫描 C: 盘 ... ");
    std::io::stdout().flush().ok();

    let wiz_start = Instant::now();
    match diskwise_scanner::wiztree::export_c_drive_csv(&wiztree, &csv_path) {
        Ok(()) => {
            let elapsed = wiz_start.elapsed();
            let csv_size = std::fs::metadata(&csv_path).map(|m| m.len()).unwrap_or(0);
            // Parse the root row to show actual data size
            let info = extract_cdrive_stats(&csv_path);
            if let Some((data_bytes, file_count)) = info {
                println!(
                    "完成 (耗时 {}, 数据量 {}, {} 文件, CSV {})",
                    format_elapsed(elapsed),
                    format_bytes(data_bytes),
                    file_count,
                    format_bytes(csv_size),
                );
            } else {
                println!(
                    "完成 (耗时 {}, CSV {})",
                    format_elapsed(elapsed),
                    format_bytes(csv_size),
                );
            }
            print_csv_head(&csv_path, 3).ok();
        }
        Err(e) => {
            eprintln!("失败: {e}");
            std::process::exit(1);
        }
    }

    // ── Walkdir 对比扫描 ────────────────────────────────────
    if let Some(path) = &walkdir_path {
        if !path.exists() {
            eprintln!("❌ 路径不存在: {}", path.display());
            std::process::exit(1);
        }

        print!("[Walkdir] 扫描 {} ... ", path.display());
        std::io::stdout().flush().ok();

        let wd_start = Instant::now();
        match diskwise_scanner::scan(path) {
            Ok(node) => {
                let elapsed = wd_start.elapsed();
                println!(
                    "完成 (耗时 {}, 数据量 {}, {} 文件)",
                    format_elapsed(elapsed),
                    format_bytes(node.size),
                    node.file_count
                );
            }
            Err(e) => {
                eprintln!("失败: {e}");
            }
        }
    }
}

/// Read the root row from WizTree CSV to get actual C: drive stats.
/// Returns (total_bytes, file_count).
fn extract_cdrive_stats(path: &std::path::Path) -> Option<(u64, u64)> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)
        .from_path(path)
        .ok()?;
    for result in rdr.records() {
        let rec = result.ok()?;
        let path_field = rec.get(0)?;
        if path_field == r"C:\" {
            let size: u64 = rec.get(1)?.parse().ok()?;
            let files: u64 = rec.get(5)?.parse().ok()?;
            return Some((size, files));
        }
    }
    None
}

fn print_csv_head(path: &std::path::Path, n: usize) -> std::io::Result<()> {
    let mut file = std::fs::File::open(path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    println!("\n  CSV 前 {n} 行:");
    for (_i, line) in buf.lines().take(n).enumerate() {
        println!("    {}", line);
    }
    let total = buf.lines().count();
    if total > n {
        println!("    ... (共 {total} 行)");
    }
    Ok(())
}

fn format_bytes(b: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = b as f64;
    let mut unit = 0;
    while size >= 1024.0 && unit < UNITS.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }
    format!("{:.1} {}", size, UNITS[unit])
}

fn format_elapsed(d: std::time::Duration) -> String {
    let secs = d.as_secs_f64();
    if secs < 60.0 {
        format!("{secs:.1}s")
    } else {
        format!("{}m{:02.0}s", secs as u64 / 60, secs % 60.0)
    }
}
