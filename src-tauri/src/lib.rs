// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use rayon::prelude::*;
use std::{
    collections::HashMap,
    ffi::OsString,
    fs,
    hash::Hasher,
    io::{BufReader, Read},
    os::windows::fs::MetadataExt,
    path::PathBuf,
};
use twox_hash::XxHash64;

const SEED: u64 = 1233135;

#[tauri::command]
fn start_analysis(path: &str) -> Result<Report, String> {
    let entries = fs::read_dir(path).unwrap();
    let mut size_map: HashMap<u64, Vec<PathBuf>> = HashMap::new();
    for entry in entries {
        if let Ok(entry) = entry {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_file() && !metadata.is_symlink() {
                    let size = metadata.file_size();
                    size_map
                        .entry(size)
                        .and_modify(|value: &mut Vec<PathBuf>| {
                            value.push(entry.path());
                        })
                        .or_insert(vec![entry.path()]);
                }
            }
        }
    }

    let size_map: HashMap<_, _> = size_map
        .into_iter()
        .filter(|(size, list)| *size != 0 && list.len() > 1)
        .collect();
    let map: HashMap<u64, HashMap<u64, Vec<PathBuf>>> = size_map
        .into_par_iter()
        .map(|(size, list)| {
            const SAMPLE_SIZE: usize = 1024;
            let sample_size = SAMPLE_SIZE.min(size as usize);
            let mut quick_hash_map = HashMap::new();
            let mut buf = vec![0; sample_size];
            list.into_iter().for_each(|p| {
                let file = fs::File::open(&p).unwrap();
                let mut reader = BufReader::new(file);
                let n = reader.read(&mut buf).unwrap();
                let hash = XxHash64::oneshot(SEED, &buf[..n]);
                quick_hash_map
                    .entry(hash)
                    .and_modify(|v: &mut Vec<_>| {
                        v.push(p.clone());
                    })
                    .or_insert(vec![p]);
            });
            let mut quick_hash_map: HashMap<u64, Vec<PathBuf>> = quick_hash_map
                .into_iter()
                .filter(|(hash, list)| list.len() > 1)
                .collect();

            (size, quick_hash_map)
        })
        .collect();
    const BATCH_SIZE: usize = 64 * 1024;
    let map: HashMap<u64, HashMap<u64, Vec<PathBuf>>> = map
        .into_iter()
        .map(|(size, quick_map)| {
            let mut long_hash_map = HashMap::new();

            for (_, list) in quick_map {
                // Compute hashes in parallel and collect results
                let hash_path_pairs: Vec<(u64, PathBuf)> = list
                    .into_par_iter()
                    .map(|p| {
                        let mut buf = vec![0; BATCH_SIZE];
                        let file = fs::File::open(&p).unwrap();
                        let mut reader = BufReader::new(file);
                        let mut hasher = XxHash64::with_seed(SEED);

                        while let Ok(n) = reader.read(&mut buf) {
                            if n == 0 {
                                break;
                            }
                            hasher.write(&buf[..n]);
                        }
                        let hash = hasher.finish();
                        (hash, p)
                    })
                    .collect();

                // Merge results into long_hash_map sequentially
                for (hash, p) in hash_path_pairs {
                    long_hash_map
                        .entry(hash)
                        .and_modify(|vec: &mut Vec<_>| {
                            vec.push(p.clone());
                        })
                        .or_insert(vec![p]);
                }
            }
            let long_hash_map:HashMap<u64, Vec<PathBuf>> = long_hash_map.into_iter().filter(|(hash,list)|{
                list.len()>1
            }).collect();
            (size, long_hash_map)
        })
        .collect();
    let mut pairs = vec![];
    for (size, hash_map) in map {
        for (hash, list) in hash_map {
            pairs.push(Record {
                hash: format!("{:016X}", hash),
                size:to_human_readable_size(size),
                files: list,
            })
        }
    }

    Ok(Report { pairs: pairs })
}


#[tauri::command]
fn remove_file(path:&str)->Result<(),String>{
    fs::remove_file(path).map_err(|e|e.to_string())
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![start_analysis,remove_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use serde::Serialize;

#[derive(Serialize)]
struct Report {
    pairs: Vec<Record>,
}
#[derive(Serialize)]
struct Record {
    hash: String,
    size: String,
    files: Vec<PathBuf>,
}

fn to_human_readable_size(size: u64) -> String {
    let size = size as f32;
    const N: f32 = 1024.0;
    if size > N {
        let k = size / N;
        if k > N {
            let m = k / N;
            if m > N {
                let g = m / N;
                if g > N {
                    let t = g / N;
                    format!("{}TB", format_significant(t, 3))
                } else {
                    format!("{}GB", format_significant(g, 3))
                }
            } else {
                format!("{}MB", format_significant(m, 3))
            }
        } else {
            format!("{}KB", format_significant(k, 3))
        }
    } else {
        format!("{}B", size)
    }
}

fn format_significant(value: f32, digits: usize) -> String {
    if value == 0.0 {
        return "0".to_string();
    }

    let log10 = value.abs().log10().floor() as i32;
    let decimals = (digits as i32 - 1 - log10).clamp(0, 15) as usize;

    format!("{0:.1$}", value, decimals)
}
