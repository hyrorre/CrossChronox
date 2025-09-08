use crate::chart::*;
use kira::AudioManager;
use macroquad::prelude::*;

#[allow(non_camel_case_types)]
#[derive(Debug, Default)]
pub enum Placement {
    #[default]
    NORMAL = 0,
    RANDOM = 1,
    R_RAN = 2,
    S_RAN = 3,
    H_RAN = 4,
    MIRROR = 5,
    SYNC_RAN = 6,
    SYM_RAN = 7,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Default)]
pub enum Gauge {
    #[default]
    NORMAL,
    A_EASY,
    EASY,
    HARD,
    EX_HARD,
    HAZARD,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Default)]
pub enum Assist {
    #[default]
    OFF,
    A_SCRATCH,
    FIVE_KEYS,
    LEGACY,
    A_SCR_5KEYS,
    A_SCR_LEGACY,
    LEGACY_5KEYS,
    FULL_ASSIST,
    AUTO_PLAY,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Default)]
pub enum DisplayArea {
    #[default]
    OFF,
    SUD,
    HID,
    SUD_HID,
    LIFT,
    LIFT_SUD,
}

#[derive(Debug, Default)]
pub struct PlayOption {
    pub placement_left: Placement,
    pub placement_right: Placement,
    pub gauge: Gauge,
    pub assist: Assist,
    pub display_area: DisplayArea,
    pub flip: bool,
}

#[derive(Debug, Default)]
pub struct NoteJudge {
    pub lane: i32,
    pub num: u64,
    pub ms: u64,
    pub judge: Judge,
    pub diff: f64,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Default)]
pub enum Lamp {
    #[default]
    NOPLAY,
    FAILED,
    ASSIST,
    EASY,
    NORMAL,
    HARD,
    EX_HARD,
    FC,
    PFC,
    MFC,
}

#[derive(Debug, Default)]
pub struct Result {
    pub pgreat_early: i32,
    pub great_early: i32,
    pub good_early: i32,
    pub bad_early: i32,
    pub poor_early: i32,
    pub kpoor_early: i32,
    pub pgreat_late: i32,
    pub great_late: i32,
    pub good_late: i32,
    pub bad_late: i32,
    pub poor_late: i32,
    pub kpoor_late: i32,
    pub combo_break: i32,
    pub miss_count: i32,
    pub max_combo: i32,
    pub now_combo: i32,
    pub note_judges: Vec<NoteJudge>,
}

impl Result {
    pub fn pgreat(&self) -> i32 {
        self.pgreat_early + self.pgreat_late
    }

    pub fn great(&self) -> i32 {
        self.great_early + self.great_late
    }

    pub fn good(&self) -> i32 {
        self.good_early + self.good_late
    }

    pub fn bad(&self) -> i32 {
        self.bad_early + self.bad_late
    }

    pub fn poor(&self) -> i32 {
        self.poor_early + self.poor_late
    }

    pub fn kpoor(&self) -> i32 {
        self.kpoor_early + self.kpoor_late
    }

    pub fn push(&mut self, note_judge: NoteJudge) {
        match note_judge.judge {
            Judge::PGREAT_EARLY => {
                self.pgreat_early += 1;
            }
            Judge::GREAT_EARLY => {
                self.great_early += 1;
            }
            Judge::GOOD_EARLY => {
                self.good_early += 1;
            }
            Judge::BAD_EARLY => {
                self.bad_early += 1;
                self.combo_break += 1;
                self.miss_count += 1;
            }
            Judge::POOR_EARLY => {
                self.poor_early += 1;
                self.combo_break += 1;
                self.miss_count += 1;
            }
            Judge::KPOOR_EARLY => {
                self.kpoor_early += 1;
                self.miss_count += 1;
            }
            Judge::PGREAT_LATE => {
                self.pgreat_late += 1;
            }
            Judge::GREAT_LATE => {
                self.great_late += 1;
            }
            Judge::GOOD_LATE => {
                self.good_late += 1;
            }
            Judge::BAD_LATE => {
                self.bad_late += 1;
                self.combo_break += 1;
                self.miss_count += 1;
            }
            Judge::POOR_LATE => {
                self.poor_late += 1;
                self.combo_break += 1;
                self.miss_count += 1;
            }
            Judge::KPOOR_LATE => {
                self.kpoor_late += 1;
                self.miss_count += 1;
            }
            _ => {}
        };
        self.note_judges.push(note_judge);
    }
}

#[derive(Default)]
pub struct Player {
    pub chart: Chart,
    pub option: PlayOption,
    pub result: Result,
    pub start_ms: u64,
    pub play_ms: u64,
    pub last_play_ms: u64,
    pub now_pulse: u64,
    pub last_pulse: u64,
    pub lane_timelines: [Vec<usize>; MAX_LANE],
}

impl Player {
    pub fn init(&mut self, chart: Chart) {
        self.chart = chart;
        self.result = Default::default();
        self.start_ms = (get_time() * 1000.0) as u64;
        self.play_ms = (get_time() * 1000.0) as u64;
        self.last_play_ms = 0;
        self.now_pulse = 0;
        self.last_pulse = 0;
        self.lane_timelines = Default::default();
    }

    pub fn update(&mut self, audio_manager: &mut AudioManager) {
        // Update the play state
        self.last_play_ms = self.play_ms;
        self.play_ms = (get_time() * 1000.0) as u64 - self.start_ms;
        self.last_pulse = self.now_pulse;
        self.now_pulse = self.chart.ms_to_pulse(self.play_ms);

        if matches!(self.option.assist, Assist::AUTO_PLAY) || true {
            let mut note_tmp = Note::default();
            note_tmp.pulse = self.last_pulse;
            for note in self
                .chart
                .notes
                .iter_mut()
                .filter(|note| note_tmp.pulse < note.pulse)
            {
                if self.now_pulse < note.pulse {
                    break;
                }
                if let Some(sound) = self.chart.sounds[note.index].clone() {
                    audio_manager.play(sound).ok();
                }
                if note.num != 0 {
                    self.result.push(NoteJudge {
                        lane: note.lane,
                        num: note.num,
                        ms: self.play_ms,
                        judge: Judge::PGREAT_EARLY,
                        diff: 0.0,
                    });
                }
            }
        } else {
            // normal play
        }
    }

    pub fn set_lane_timelines(&mut self) {
        // Set the lane timelines
        for lane_timeline in self.lane_timelines.iter_mut() {
            lane_timeline.clear();
        }

        let placements = match self.chart.info.mode {
            Mode::BEAT_5K => vec![1, 2, 3, 4, 5],
            Mode::BEAT_7K => vec![1, 2, 3, 4, 5, 6, 7],
            _ => vec![],
        };

        for i in 0..self.chart.notes.len() {
            let note = &self.chart.notes[i];
            let lane_origin = note.lane_origin;
            if 1 <= lane_origin && lane_origin <= 7 {
                let lane = placements[note.lane_origin as usize - 1];
                self.chart.notes[i].lane = lane;
                self.lane_timelines[lane as usize].push(i);
            } else {
                self.lane_timelines[lane_origin as usize].push(i);
            }
        }
    }
}
