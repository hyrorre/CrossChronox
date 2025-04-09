#![allow(dead_code)]
use regex::RegexBuilder;
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

use crate::chart::*;

const BEAT_RESOLUTION: u64 = 240;
const MAX_BAR_INFO: usize = 1001;
const MAX_NOTE_CHANNEL: usize = 30;

const CHANNEL_BGM: usize = 1;
const CHANNEL_METER: usize = 2;
const CHANNEL_BPM: usize = 3;
const CHANNEL_BGABASE: usize = 4;
const CHANNEL_BGAPOOR: usize = 6;
const CHANNEL_BGALAYER: usize = 7;
const CHANNEL_EXBPM: usize = 8;
const CHANNEL_STOPS: usize = 9;

type ChannelType = i32;
const CHANNEL_TYPE_BMS: ChannelType = 0;
const CHANNEL_TYPE_PMS: ChannelType = 1;
const CHANNEL_TYPE_PME: ChannelType = 2;

struct BarInfo {
    scale: f64,
    start_pulse: u64,
}

impl BarInfo {
    fn new() -> BarInfo {
        return BarInfo {
            scale: 1.0,
            start_pulse: 0,
        };
    }

    fn length(&self) -> u64 {
        return (self.scale * BEAT_RESOLUTION as f64 * 4.0) as u64;
    }
}

struct TmpNoteData {
    bar: i32,
    channel: i32,
    index: i32,
    lnend: bool,
    bar_pulse: u64,
    global_pulse: u64,
}

impl TmpNoteData {
    fn new() -> TmpNoteData {
        return TmpNoteData {
            bar: 0,
            channel: 0,
            index: 0,
            lnend: false,
            bar_pulse: 0,
            global_pulse: 0,
        };
    }
}

impl Ord for TmpNoteData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.global_pulse.cmp(&other.global_pulse)
    }
}

impl PartialOrd for TmpNoteData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for TmpNoteData {
    fn eq(&self, other: &Self) -> bool {
        self.global_pulse == other.global_pulse
    }
}

impl Eq for TmpNoteData {}

fn index(nowline: &String) -> i32 {
    // #BPM01
    let mut bytes = nowline.bytes();
    let index_str = String::from_utf8(vec![bytes.nth(4).unwrap_or(0), bytes.nth(5).unwrap_or(0)])
        .unwrap_or("00".to_string());
    index_str.parse().unwrap_or(0)
}

fn arg(nowline: &String) -> String {
    nowline.split_whitespace().nth(1).unwrap_or("0").to_string()
}

fn channel_to_lane(channel_type: ChannelType, mut channel: i32) -> i32 {
    if channel_type == CHANNEL_TYPE_BMS {
        if 51 <= channel {
            channel -= 40;
        }

        match channel as usize {
            CHANNEL_BGM => 0,
            11..=15 => channel - 10, // 1P KEY1-5
            16 => 8,                 // SC
            // case 17: //FREEZONE
            18..=19 => channel - 12, // 1P KEY6-7
            21..=25 => channel - 12, // 2P KEY1-5
            26 => 16,                // SC
            // case 27: //FREEZONE
            28..=29 => channel - 14, // 2P KEY6-7
            _ => -1,
        }
    } else if channel_type == CHANNEL_TYPE_PMS {
        if 51 <= channel {
            channel -= 40;
        }
        match channel as usize {
            CHANNEL_BGM => 0,
            11..=15 => channel - 10, // KEY1-5
            22..=25 => channel - 16, // KEY6-9
            _ => -1,
        }
    } else if channel_type == CHANNEL_TYPE_PME {
        if 51 <= channel {
            channel -= 40;
        }
        match channel as usize {
            CHANNEL_BGM => 0,
            11..=15 => channel - 10, // KEY1-5
            18..=19 => channel - 12, // KEY6-7
            16..=17 => channel - 8,  // KEY8-9
            _ => -1,
        }
    } else {
        -1
    }
}

pub fn load_bms(filename: &str, load_header_only_flag: bool) -> Result<ChartData, Box<dyn Error>> {
    let mut bar_info: [BarInfo; MAX_BAR_INFO] = core::array::from_fn(|_| BarInfo::new());
    let mut max_bar: usize = 0;
    let mut tmp_notes: Vec<TmpNoteData> = Vec::new();
    let mut exbpm: HashMap<i32, f64> = HashMap::new();
    let mut stop: HashMap<i32, u64> = HashMap::new();
    let mut lnobj: Vec<i32> = Vec::new();
    let mut channel_used_flag: [bool; MAX_NOTE_CHANNEL] = [false; MAX_NOTE_CHANNEL];

    let mut random_num: i32 = 0;
    let mut parse_nextline_flag = true;

    let mut chart = ChartData::new();

    let extention = PathBuf::from(filename)
        .extension()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default()
        .to_string();

    chart.bpm_events.push(BpmEvent::new());

    // open file
    let buf_u8 = std::fs::read(filename)?;
    let buf_string = encoding_rs::SHIFT_JIS.decode(&buf_u8).0.into_owned();

    for (_line_num, line) in buf_string.lines().enumerate() {
        let mut nowline = line.to_string();
        if nowline.starts_with("＃") {
            nowline = nowline.replacen("＃", "#", 1);
        }
        if nowline.starts_with("#") {
            // Control Flow (Nested #RANDOM is not support.)
            if nowline.to_uppercase().starts_with("#IF") {
                let random_num2: i32 = arg(&nowline).parse().unwrap_or(1);
                parse_nextline_flag = random_num == random_num2;
            } else if nowline.to_uppercase().starts_with("#END") {
                parse_nextline_flag = true;
            } else if nowline.to_uppercase().starts_with("#RANDOM") {
                let random_max: i32 = arg(&nowline).parse().unwrap_or(1);
                let mut rng = rand::rng();
                random_num = rand::Rng::random_range(&mut rng, 1..=random_max);
                chart.info.random_flag = true;
            } else if parse_nextline_flag {
                if !load_header_only_flag && nowline.to_uppercase().starts_with("#WAV") {
                    // Load WAV
                } else if nowline.to_uppercase().starts_with("#BPM") {
                    if let Some(c) = nowline.bytes().nth(4) {
                        if '0' <= c as char && c as char <= '9' {
                            exbpm.insert(index(&nowline), arg(&nowline).parse().unwrap_or(130.0));
                        } else {
                            let bpm: f64 = arg(&nowline).parse().unwrap_or(130.0);
                            chart.info.init_bpm = bpm;
                            chart.bpm_events.clear();
                            chart.bpm_events.push(BpmEvent {
                                event_type: BPM_CHANGE,
                                pulse: 0,
                                bpm: bpm,
                                duration: 0,
                                ms: 0,
                            });
                        }
                    }
                } else if nowline.to_uppercase().starts_with("#LNOBJ") {
                    lnobj.push(i32::from_str_radix(arg(&nowline).as_str(), 36).unwrap_or(-1));
                } else if nowline.to_uppercase().starts_with("#STOP") {
                    stop.insert(index(&nowline), arg(&nowline).parse().unwrap_or(0));
                } else if nowline.to_uppercase().starts_with("#TITLE") {
                    chart.info.title = arg(&nowline);
                } else if nowline.to_uppercase().starts_with("#SUBTITLE") {
                    chart.info.subtitle = arg(&nowline);
                } else if nowline.to_uppercase().starts_with("#ARTIST") {
                    chart.info.artist = arg(&nowline);
                } else if nowline.to_uppercase().starts_with("#SUBARTIST") {
                    chart.info.subartist = arg(&nowline);
                } else if nowline.to_uppercase().starts_with("#GENRE") {
                    chart.info.genre = arg(&nowline);
                } else if nowline.to_uppercase().starts_with("#TOTAL") {
                    chart.info.total = arg(&nowline).parse().unwrap_or(0.0);
                } else if nowline.to_uppercase().starts_with("#BACKBMP") {
                    chart.info.back_image = arg(&nowline);
                } else if nowline.to_uppercase().starts_with("#STAGEFILE") {
                    chart.info.eyecatch_image = arg(&nowline);
                } else if nowline.to_uppercase().starts_with("#BANNER") {
                    chart.info.banner_image = arg(&nowline);
                } else if nowline.to_uppercase().starts_with("#PREVIEW") {
                    chart.info.preview_music = arg(&nowline);
                } else if nowline.to_uppercase().starts_with("#PLAYLEVEL") {
                    chart.info.level = arg(&nowline).parse().unwrap_or(0);
                } else if nowline.to_uppercase().starts_with("#DIFFICULTY") {
                    chart.info.difficulty = arg(&nowline).parse().unwrap_or(0);
                    chart.info.chart_name = match chart.info.difficulty {
                        0 => "UNDEFINED",
                        1 => "BEGINNER",
                        2 => "NORMAL",
                        3 => "HYPER",
                        4 => "ANOTHER",
                        5 => "INSANE",
                        _ => "UNDEFINED",
                    }
                    .to_string();
                } else if nowline.to_uppercase().starts_with("#RANK") {
                    chart.info.rank = arg(&nowline).parse().unwrap_or(0).clamp(0, 3);
                } else if nowline.to_uppercase().starts_with("#BASEBPM") {
                    chart.info.base_bpm = arg(&nowline).parse().unwrap_or(0.0);
                } else {
                    // obj line?
                    let mut chars = nowline.chars();
                    if chars.nth(6).unwrap_or_default() != ':' {
                        // '#nnncc:'でないので処理不能
                        continue;
                    }
                    for i in [1, 2, 3, 4, 5] {
                        //'#nnncc:'でないので処理不能
                        if !chars.nth(i).unwrap_or('-').is_alphanumeric() {
                            continue;
                        }
                    }
                    // start parsing
                    let bar: usize = nowline[1..4].parse().unwrap_or(0);
                    let channel: usize = nowline[4..6].parse().unwrap_or(0);
                    max_bar = max_bar.max(bar);

                    let arg = nowline[7..].trim_start().to_string();

                    // set channel_used_flag
                    if 11 <= channel {
                        if channel <= 29 {
                            channel_used_flag[channel] = true;
                        } else if channel <= 49 {
                            channel_used_flag[channel - 20] = true;
                        } else if channel <= 69 {
                            channel_used_flag[channel - 40] = true;
                        }
                    }

                    if channel == CHANNEL_METER {
                        bar_info[bar].scale = arg.parse().unwrap_or(1.0);
                    } else {
                        let mut len = 0;
                        let base = if channel == CHANNEL_BPM { 16 } else { 36 };
                        while len < arg.len()
                            && arg.chars().nth(len).unwrap_or(' ').is_alphanumeric()
                        {
                            len += 1;
                        }
                        let resolution = len / 2;
                        for i in 0..resolution {
                            let index =
                                i32::from_str_radix(&arg[i * 2..i * 2 + 2], base).unwrap_or(0);
                            if index != 0 {
                                tmp_notes.push(TmpNoteData {
                                    bar: bar as i32,
                                    channel: channel as i32,
                                    index,
                                    lnend: false,
                                    bar_pulse: 4 * BEAT_RESOLUTION * i as u64 / resolution as u64,
                                    global_pulse: 0,
                                });
                            }
                        }
                    }
                }
            }
        }
    } // end reading file

    // set subtitle
    if chart.info.subtitle.is_empty() {
        let regex =
            RegexBuilder::new("(-[^-]+-|~.+~|<.+>|\\(.+\\)|\\[.+\\]|～.+～|\".+\")$").build()?;
        regex.captures(&chart.info.title.clone()).map(|caps| {
            if let Some(m) = caps.get(0) {
                chart.info.subtitle = m.as_str().to_string();
                chart.info.title = chart.info.title[..m.start()].trim_end().to_string();
            }
        });
    }

    // set mode
    let mut used_channel_max = MAX_NOTE_CHANNEL - 1;
    loop {
        if channel_used_flag[used_channel_max] {
            break;
        }
        if used_channel_max == 11 {
            break;
        }
        used_channel_max -= 1;
    }

    let channel_type: ChannelType;

    // bms, bme, bml
    if extention.to_lowercase().starts_with("bm") {
        channel_type = CHANNEL_TYPE_BMS;
        if 28 <= used_channel_max {
            chart.info.mode = BEAT_14K;
        } else if 21 <= used_channel_max {
            if channel_used_flag[18] || channel_used_flag[19] {
                chart.info.mode = BEAT_14K;
            } else {
                chart.info.mode = BEAT_10K;
            }
        } else if 18 <= used_channel_max {
            chart.info.mode = BEAT_7K;
        } else {
            chart.info.mode = BEAT_5K;
        }
    }
    // pms, pme(pms BME-TYPE)
    else {
        if 24 <= used_channel_max {
            chart.info.mode = POPN_9K;
            channel_type = CHANNEL_TYPE_PMS;
        } else if 22 <= used_channel_max {
            if channel_used_flag[11] || channel_used_flag[12] {
                chart.info.mode = POPN_9K;
                channel_type = CHANNEL_TYPE_PMS;
            } else {
                chart.info.mode = POPN_5K;
                channel_type = CHANNEL_TYPE_PMS;
            }
        } else {
            chart.info.mode = POPN_9K;
            channel_type = CHANNEL_TYPE_PME;
        }
    }

    // set bar_info
    max_bar += 1;
    if max_bar >= MAX_BAR_INFO {
        return Err("Too many bars. (max_bar > 1000)".into());
    }
    let mut total_pulse: u64 = 0;
    for i in 0..=max_bar {
        bar_info[i].start_pulse = total_pulse;
        total_pulse += bar_info[i].length();
        chart.lines.push(BarLine::new_with_pulse(total_pulse));
    }

    // set bar_pulse and global_pulse of tmp_notes
    for tmp_note in &mut tmp_notes {
        tmp_note.global_pulse = bar_info[tmp_note.bar as usize].start_pulse + tmp_note.bar_pulse;
    }

    // sort tmp_notes
    tmp_notes.sort();

    // sort lnobj
    lnobj.sort();

    let mut note_count = 0;

    // tmp_notes -> ScoreData
    let mut ln_pushing: [bool; MAX_LANE] = [false; MAX_LANE];
    let mut last_note_index: [Option<usize>; MAX_LANE] = [None; MAX_LANE]; // last note of the lane

    for tmp_note in &tmp_notes {
        match tmp_note.channel as usize {
            CHANNEL_METER => {
                // assert(false);
            }
            CHANNEL_BGABASE | CHANNEL_BGAPOOR | CHANNEL_BGALAYER => {
                // ignore //TODO: implement
            }
            CHANNEL_BPM => {
                chart.bpm_events.push(BpmEvent {
                    event_type: BPM_CHANGE,
                    pulse: tmp_note.global_pulse,
                    bpm: tmp_note.index as f64,
                    duration: 0,
                    ms: 0,
                });
            }
            CHANNEL_EXBPM => {
                if let Some(bpm) = exbpm.get(&tmp_note.index) {
                    chart.bpm_events.push(BpmEvent {
                        event_type: BPM_CHANGE,
                        pulse: tmp_note.global_pulse,
                        bpm: *bpm,
                        duration: 0,
                        ms: 0,
                    });
                } else {
                    // default of exBPM is 120
                    chart.bpm_events.push(BpmEvent {
                        event_type: BPM_CHANGE,
                        pulse: tmp_note.global_pulse,
                        bpm: 120.0,
                        duration: 0,
                        ms: 0,
                    });
                }
            }
            CHANNEL_STOPS => {
                if let Some(stop_time) = stop.get(&tmp_note.index) {
                    chart.bpm_events.push(BpmEvent {
                        event_type: STOP,
                        pulse: tmp_note.global_pulse,
                        bpm: 10.0 * *stop_time as f64,
                        duration: 0,
                        ms: 0,
                    });
                } else {
                    // do nothing
                    // return Err("Some errors are found. (#STOP)".into());
                }
            }
            _ => {
                // Normal note, LN (invisible note is not implemented yet)
                let ln_channel_flag = 51 <= tmp_note.channel && tmp_note.channel <= 69;
                let lane = channel_to_lane(channel_type, tmp_note.channel) as usize;
                let mut lnend_flag = false;

                if ln_channel_flag {
                    if ln_pushing[lane] {
                        lnend_flag = true;
                    }
                    ln_pushing[lane] = !ln_pushing[lane];
                } else if lnobj.binary_search(&tmp_note.index).is_ok() {
                    lnend_flag = true;
                }

                if lnend_flag {
                    if let Some(i) = last_note_index[lane] {
                        if let Some(last) = chart.notes.get_mut(i) {
                            last.len = tmp_note.global_pulse - last.pulse;
                        }
                    }
                    continue;
                }

                if lane < MAX_LANE {
                    note_count += 1;
                    let num = if lane > 0 { note_count } else { 0 };
                    chart.notes.push(Note {
                        lane: lane as i32,
                        lane_origin: lane as i32,
                        pulse: tmp_note.global_pulse,
                        len: 0,
                        num: num as u64,
                        judge: JUDGE_YET,
                        empty_poor_count: 0,
                        ms: 0,
                        lnend_ms: 0,
                    });
                    last_note_index[lane] = Some(chart.notes.len() - 1);
                }
            }
        }
    }
    // set note_count
    chart.info.note_count = note_count;

    // set bpm
    let init = chart.info.init_bpm;
    chart.info.max_bpm = init;
    chart.info.min_bpm = init;

    // bpm_length (key: bpm x 1000_000, value: length pulse)
    let mut bpm_length: HashMap<u64, u64> = HashMap::new();
    let mut last_bpm_change_i = 0;

    for i in 1..chart.bpm_events.len() {
        if chart.bpm_events[i].event_type == BPM_CHANGE {
            let duration = chart.bpm_events[i].pulse - chart.bpm_events[last_bpm_change_i].pulse;
            chart.bpm_events[i - 1].duration = duration;
            bpm_length
                .entry((chart.bpm_events[i - 1].bpm * 1000_000.0) as u64)
                .and_modify(|e| *e += (duration as f64 * chart.bpm_events[i - 1].bpm) as u64)
                .or_insert((duration as f64 * chart.bpm_events[i - 1].bpm) as u64);

            chart.info.max_bpm = chart.info.max_bpm.max(chart.bpm_events[i].bpm);
            chart.info.min_bpm = chart.info.min_bpm.min(chart.bpm_events[i].bpm);
            last_bpm_change_i = i;
        } else {
            chart.bpm_events[i].bpm = chart.bpm_events[last_bpm_change_i].bpm;
        }
        chart.bpm_events[i].ms = chart.bpm_events[last_bpm_change_i]
            .next_event_ms(chart.bpm_events[i].pulse, chart.info.resolution);
    }
    // set end_pulse
    chart.info.end_pulse = tmp_notes.last().unwrap_or(&TmpNoteData::new()).global_pulse;
    bpm_length
        .entry((chart.bpm_events[chart.bpm_events.len() - 1].bpm * 1000_000.0) as u64)
        .and_modify(|e| {
            *e += ((chart.info.end_pulse - chart.bpm_events[chart.bpm_events.len() - 1].pulse)
                as f64
                * chart.bpm_events[chart.bpm_events.len() - 1].bpm) as u64
        });

    let (base_bpm_x_1000_000, _) = bpm_length.iter().max().unwrap_or((&0, &0));
    chart.info.base_bpm = *base_bpm_x_1000_000 as f64 / 1000_000.0;

    // set note time
    let mut j = 0;
    for i in 0..chart.notes.len() {
        while j < chart.bpm_events.len() - 1
            && chart.bpm_events[j].pulse + chart.bpm_events[j].duration < chart.notes[i].pulse
        {
            j += 1;
        }
        chart.notes[i].ms =
            chart.bpm_events[j].next_event_ms(chart.notes[i].pulse, chart.info.resolution);
        if chart.notes[i].len != 0 {
            let lnend_pulse = chart.notes[i].pulse + chart.notes[i].len;
            let mut j_end = j + 1;
            while j_end < chart.bpm_events.len() && chart.bpm_events[j_end].pulse < lnend_pulse {
                j_end += 1;
            }
            chart.notes[i].lnend_ms =
                chart.bpm_events[j_end - 1].next_event_ms(lnend_pulse, chart.info.resolution);
        }
    }

    chart.info.end_ms = chart
        .bpm_events
        .last()
        .unwrap_or(&BpmEvent::new())
        .next_event_ms(chart.info.end_pulse, chart.info.resolution);

    if !load_header_only_flag {
        // load wavs
    }

    Ok(chart)
}
