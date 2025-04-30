use async_recursion::async_recursion;

use crate::chart::*;
use std::fs::*;
use std::path::*;

// use super::bms_loader::load_bms;

#[derive(Debug)]
pub enum ChartTreeItem {
    ChartInfo(ChartInfo),
    ChartTree(ChartTree),
}

#[derive(Debug, Default)]
pub struct ChartTree {
    pub path: PathBuf,
    pub items: Vec<ChartTreeItem>,
}

impl ChartTree {
    pub async fn new(path: PathBuf) -> ChartTree {
        return ChartTree {
            path,
            items: Vec::new(),
        };
    }

    #[async_recursion]
    pub async fn load(&mut self) {
        let Ok(entries) = read_dir(&self.path) else {
            return;
        };
        for entry_result in entries {
            let Ok(entry) = entry_result else {
                continue;
            };
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            if is_chart_dir(path.as_path()) {
                let mut chart_tree = ChartTree::new(path).await;
                chart_tree.load_chart_dir().await;
                self.items.push(ChartTreeItem::ChartTree(chart_tree));
            } else {
            }
        }
    }

    pub async fn load_chart_dir(&mut self) {
        // for (fs::directory_iterator it(path); it != end; ++it) {
        //     if (fs::is_directory(*it))
        //         continue;
        //     std::string file_extention = it->path().extension().string();
        //     if (!file_extention.empty() && file_extention.front() == '.')
        //         file_extention = file_extention.substr(1);
        //     for (const auto& extention : bms_extentions) {
        //         if (file_extention == extention) {
        //             LoadBms(it->path().string(), &tmp_data, true);
        //             out->children.emplace_back(new ScoreInfo(tmp_data.info));
        //         }
        //     }
        // }
    }
}

fn is_chart_dir(path: &Path) -> bool {
    let Ok(entries) = read_dir(path) else {
        return false;
    };
    for entry_result in entries {
        let Ok(entry) = entry_result else {
            continue;
        };
        let path = entry.path();
        let extension = path
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();
        match extension {
            "bms" | "bme" | "bml" | "BMS" | "BME" | "BML" => return true,
            _ => {}
        }
    }
    return false;
}
