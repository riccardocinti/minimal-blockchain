use crate::blockchain::transaction::Transaction;
use crate::node::node::Node;
use std::io::Error;
use std::path::Path;

#[derive(Debug, Default)]
pub struct IpcIngestStats {
    pub accepted: usize,
    pub rejected: usize,
    pub failed: usize,
}

pub fn ingest_ipc_transactions(
    node: &mut Node,
    tx_pool_dir: &Path,
) -> Result<IpcIngestStats, Error> {
    let mut stats = IpcIngestStats::default();

    if !tx_pool_dir.exists() {
        return Ok(stats);
    }

    for entry in tx_pool_dir.read_dir()? {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                eprintln!("Failed to read tx pool dir entry: {}", e);
                stats.failed += 1;
                continue;
            }
        };

        let entry_path = entry.path();

        if !is_tx_file(&entry_path) {
            continue;
        }

        match process_tx_file(node, &entry_path) {
            Ok(true) => stats.accepted += 1,
            Ok(false) => stats.rejected += 1,
            Err(_) => stats.failed += 1,
        }
    }

    Ok(stats)
}

fn is_tx_file(path: &Path) -> bool {
    path.is_file()
        && path
            .file_name()
            .and_then(|n| n.to_str())
            .map(|n| n.starts_with("tx_") && n.ends_with(".json"))
            .unwrap_or(false)
}
fn process_tx_file(node: &mut Node, path: &Path) -> Result<bool, Error> {
    let bytes = std::fs::read(path)?;
    let tx: Transaction = match serde_json::from_slice(&bytes) {
        Ok(tx) => tx,
        Err(e) => {
            eprintln!("Failed to parse tx file {}: {}", path.display(), e);
            let _ = std::fs::remove_file(path);
            return Ok(false);
        }
    };

    node.submit_transaction(tx);

    std::fs::remove_file(path)?;

    Ok(true)
}
