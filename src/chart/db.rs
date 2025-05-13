use std::{fs::read_dir, path::Path};

use macroquad::prelude::*;
use rusqlite::*;

use super::{ChartInfo, bms_loader::load_bms};

pub fn init_songs_db() -> Result<Connection, rusqlite::Error> {
    let connection = Connection::open("assets/songs.db")?;

    connection.execute(
        "create table if not exists folder (
            path text primary key,
            parent text,
            title text,
            subtitle text
        )",
        [],
    )?;

    // TODO: not null
    connection.execute(
        "create table if not exists song (
            path text primary key,
            folder text,
            parent text,
            title text,
            subtitle text,
            artist text,
            subartist text,
            genre text,
            mode integer,
            ln_type integer,
            difficulty integer,
            level integer,
            rank integer,
            total integer,
            back_image text,
            eyecatch_image text,
            banner_image text,
            preview_music text,
            resolution integer,
            end_pulse integer,
            end_ms integer,
            init_bpm integer,
            max_bpm integer,
            min_bpm integer,
            base_bpm integer,
            note_count integer,
            feature integer,
            md5 text,
            sha256 text
        )",
        [],
    )?;

    Ok(connection)
}

pub fn clear_cache(connection: &Connection) -> Result<(), rusqlite::Error> {
    connection.execute("delete from song", [])?;
    connection.execute("delete from folder", [])?;
    Ok(())
}

pub fn load_folder<P: AsRef<Path>>(
    connection: &Connection,
    path: P,
) -> Result<(), rusqlite::Error> {
    let Ok(dir) = read_dir(path.as_ref()) else {
        return Err(rusqlite::Error::InvalidPath(path.as_ref().to_path_buf()));
    };
    for entry in dir.filter_map(|entry| entry.ok()) {
        if !entry.path().is_dir() {
            continue;
        }

        let result = connection.execute(
            "insert into folder (\
                path,\
                parent,\
                title,\
                subtitle\
            ) values (\
                ?1,\
                ?2,\
                ?3,\
                ?4\
            )",
            params![
                match entry.path().to_str() {
                    Some(path) => path,
                    None => {
                        warn!("Failed to convert path to string : {:?}", entry.path());
                        continue;
                    }
                },
                match entry.path().parent() {
                    Some(parent) => match parent.to_str() {
                        Some(parent) => parent,
                        None => {
                            warn!("Failed to convert path to string : {:?}", parent);
                            continue;
                        }
                    },
                    None => continue,
                },
                match entry.file_name().to_str() {
                    Some(title) => title,
                    None => {
                        warn!("Failed to convert path to string : {:?}", entry.file_name());
                        continue;
                    }
                },
                "",
            ],
        );
        if let Err(e) = result {
            warn!("Failed to insert folder into database: {:?}", e);
            continue;
        }

        let chart_infos = load_chart_dir(entry.path().as_path());
        if chart_infos.is_empty() {
            load_folder(connection, &entry.path())?;
        } else {
            for chart_info in chart_infos {
                let result = connection.execute(
                    "insert into song (
                        path,\
                        folder,\
                        parent,\
                        title,\
                        subtitle,\
                        artist,\
                        subartist,\
                        genre,\
                        mode,\
                        ln_type,\
                        difficulty,\
                        level,\
                        rank,\
                        total,\
                        back_image,\
                        eyecatch_image,\
                        banner_image,\
                        preview_music,\
                        resolution,\
                        end_pulse,\
                        end_ms,\
                        init_bpm,\
                        max_bpm,\
                        min_bpm,\
                        base_bpm,\
                        note_count,\
                        feature,\
                        md5,\
                        sha256\
                    ) values (\
                        ?1,\
                        ?2,\
                        ?3,\
                        ?4,\
                        ?5,\
                        ?6,\
                        ?7,\
                        ?8,\
                        ?9,\
                        ?10,\
                        ?11,\
                        ?12,\
                        ?13,\
                        ?14,\
                        ?15,\
                        ?16,\
                        ?17,\
                        ?18,\
                        ?19,\
                        ?20,\
                        ?21,\
                        ?22,\
                        ?23,\
                        ?24,\
                        ?25,\
                        ?26,\
                        ?27,\
                        ?28,\
                        ?29\
                    )",
                    params![
                        match chart_info.path.to_str() {
                            Some(path) => path,
                            None => {
                                warn!("Failed to convert path to string : {:?}", chart_info.path);
                                continue;
                            }
                        },
                        match chart_info.folder.to_str() {
                            Some(folder) => folder,
                            None => {
                                warn!("Failed to convert path to string : {:?}", chart_info.folder);
                                continue;
                            }
                        },
                        match chart_info.parent.to_str() {
                            Some(parent) => parent,
                            None => {
                                warn!("Failed to convert path to string : {:?}", chart_info.parent);
                                continue;
                            }
                        },
                        chart_info.title,
                        chart_info.subtitle,
                        chart_info.artist,
                        chart_info.subartist,
                        chart_info.genre,
                        chart_info.mode as i32,
                        chart_info.ln_type as i32,
                        chart_info.difficulty,
                        chart_info.level,
                        chart_info.rank as i32,
                        chart_info.total,
                        chart_info.back_image,
                        chart_info.eyecatch_image,
                        chart_info.banner_image,
                        chart_info.preview_music,
                        chart_info.resolution,
                        chart_info.end_pulse,
                        chart_info.end_ms,
                        chart_info.init_bpm,
                        chart_info.max_bpm,
                        chart_info.min_bpm,
                        chart_info.base_bpm,
                        chart_info.note_count,
                        chart_info.feature,
                        chart_info.md5,
                        chart_info.sha256,
                    ],
                );
                if let Err(e) = result {
                    warn!("Failed to insert chart info into database: {:?}", e);
                }
            }
        }
    }
    Ok(())
}

fn load_chart_dir<P: AsRef<Path>>(path: P) -> Vec<ChartInfo> {
    let mut chart_infos = Vec::new();
    let Ok(entries) = read_dir(path) else {
        return chart_infos;
    };
    for entry_result in entries {
        let Ok(entry) = entry_result else {
            continue;
        };
        let path_buf = entry.path();
        let extension = path_buf
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();
        if let Some(chart) = match extension {
            "bms" | "bme" | "bml" | "BMS" | "BME" | "BML" => {
                load_bms(path_buf.as_path(), true).ok()
            }
            _ => None,
        } {
            chart_infos.push(chart.info);
        }
    }
    return chart_infos;
}
