#![allow(dead_code)]

pub mod bms_loader;

pub type Judge = i32;
pub const LN_PUSHING_PGREAT: Judge = -10;
pub const LN_PUSHING_GREAT: Judge = -9;
pub const LN_PUSHING_GOOD: Judge = -8;
pub const JUDGE_YET: Judge = -1;
pub const PGREAT: Judge = 0;
pub const GREAT: Judge = 1;
pub const GOOD: Judge = 2;
pub const BAD: Judge = 3;
pub const POOR: Judge = 4;
pub const MAX_JUDGE: Judge = 5;

pub type Rank = i32;
pub const VHARD: Rank = 0;
pub const HARD: Rank = 1;
pub const NORMAL: Rank = 2;
pub const EASY: Rank = 3;
pub const VEASY: Rank = 4;
pub const MAX_RANK: Rank = 5;

pub type Mode = i32;
pub const BEAT_5K: Mode = 0;
pub const BEAT_7K: Mode = 1;
pub const BEAT_10K: Mode = 2;
pub const BEAT_14K: Mode = 3;
pub const POPN_5K: Mode = 4;
pub const POPN_9K: Mode = 5;

pub type LnType = i32;
pub const LN: LnType = 0;
pub const CN: LnType = 1;
pub const HCN: LnType = 2;

pub type BpmEventType = i32;
pub const BPM_CHANGE: BpmEventType = 0;
pub const STOP: BpmEventType = 1;

pub const MAX_LANE: usize = 20;

#[derive(Debug)]
pub struct Note {
    pub lane: i32,
    pub lane_origin: i32,
    pub pulse: u64,
    pub len: u64,
    pub num: u64,
    pub judge: Judge,
    pub empty_poor_count: i32,
    pub ms: u64,
    pub lnend_ms: u64,
    // pub wavbuf_ptr: Option<WavBuffer>,
}

impl Note {
    pub fn new() -> Note {
        return Note {
            lane: 0,
            lane_origin: 0,
            pulse: 0,
            len: 0,
            num: 0,
            judge: JUDGE_YET,
            empty_poor_count: 0,
            ms: 0,
            lnend_ms: 0,
            // wavbuf_ptr: None,
        };
    }
}

#[derive(Debug)]
pub struct BarLine {
    pulse: u64,
}

impl BarLine {
    pub fn new() -> BarLine {
        return BarLine { pulse: 0 };
    }

    pub fn new_with_pulse(pulse: u64) -> BarLine {
        return BarLine { pulse };
    }
}

#[derive(Debug)]
pub struct BpmEvent {
    pub event_type: BpmEventType,
    pub pulse: u64,
    pub bpm: f64,
    pub duration: u64,
    pub ms: u64,
}

impl BpmEvent {
    pub fn new() -> BpmEvent {
        return BpmEvent {
            event_type: BPM_CHANGE,
            pulse: 0,
            bpm: 130.0,
            duration: 0,
            ms: 0,
        };
    }

    pub fn next_event_ms(&self, pulse: u64, resolution: u64) -> u64 {
        return ((self.ms + (pulse - self.pulse) * 60 * 1000) as f64
            / (self.bpm * resolution as f64)) as u64;
    }
}

#[derive(Debug)]
pub struct BGAHeader {
    pub id: i32,
    pub name: String,
}

impl BGAHeader {
    pub fn new() -> BGAHeader {
        return BGAHeader {
            id: 0,
            name: String::new(),
        };
    }
}

#[derive(Debug)]
pub struct BGAEvent {
    pub pulse: u64,
    pub id: i32,
}

impl BGAEvent {
    pub fn new() -> BGAEvent {
        return BGAEvent { pulse: 0, id: 0 };
    }
}

#[derive(Debug)]
pub struct BGA {
    bga_header: Vec<BGAHeader>,
    bga_events: Vec<BGAEvent>,
    layer_events: Vec<BGAEvent>,
    poor_events: Vec<BGAEvent>,
}

impl BGA {
    pub fn new() -> BGA {
        return BGA {
            bga_header: Vec::new(),
            bga_events: Vec::new(),
            layer_events: Vec::new(),
            poor_events: Vec::new(),
        };
    }
}

#[derive(Debug)]
pub struct ChartInfo {
    pub title: String,
    pub subtitle: String,
    pub artist: String,
    pub subartist: String,
    pub genre: String,
    pub mode: Mode,
    pub ln_type: LnType,
    pub chart_name: String,
    pub difficulty: i32,
    pub level: i32,
    pub init_bpm: f64,
    pub rank: Rank,
    pub total: f32,
    pub back_image: String,
    pub eyecatch_image: String,
    pub banner_image: String,
    pub preview_music: String,
    pub resolution: u64,
    pub end_pulse: u64,
    pub end_ms: u64,
    pub max_bpm: f64,
    pub min_bpm: f64,
    pub base_bpm: f64,
    pub note_count: i32,
    pub md5: String,
    pub random_flag: bool,
}

impl ChartInfo {
    pub fn new() -> ChartInfo {
        return ChartInfo {
            title: String::new(),
            subtitle: String::new(),
            artist: String::new(),
            subartist: String::new(),
            genre: String::new(),
            mode: BEAT_5K,
            ln_type: LN,
            chart_name: String::new(),
            difficulty: 0,
            level: 0,
            init_bpm: 130.0,
            rank: NORMAL,
            total: 0.0,
            back_image: String::new(),
            eyecatch_image: String::new(),
            banner_image: String::new(),
            preview_music: String::new(),
            resolution: 240,
            end_pulse: 0,
            end_ms: 0,
            max_bpm: 0.0,
            min_bpm: 0.0,
            base_bpm: 130.0,
            note_count: 0,
            md5: String::new(),
            random_flag: false,
        };
    }
}

#[derive(Debug)]
pub struct ChartData {
    pub version: String,
    pub info: ChartInfo,
    pub lines: Vec<BarLine>,
    pub bpm_events: Vec<BpmEvent>,
    pub bga: BGA,
    pub notes: Vec<Note>,
    // pub wavbufs: Vec<WavBuffer>,
}

impl ChartData {
    pub fn new() -> ChartData {
        return ChartData {
            version: String::new(),
            info: ChartInfo::new(),
            lines: Vec::new(),
            bpm_events: Vec::new(),
            bga: BGA::new(),
            notes: Vec::new(),
            // wavbufs: Vec::new(),
        };
    }
}
