use crate::chart::*;
use crate::scene::State;
use crate::system::time::*;

#[allow(non_camel_case_types)]
pub enum Placement {
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
pub enum Gauge {
    NORMAL,
    A_EASY,
    EASY,
    HARD,
    EX_HARD,
    HAZARD,
}

#[allow(non_camel_case_types)]
pub enum Assist {
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
pub enum DisplayArea {
    OFF,
    SUD,
    HID,
    SUD_HID,
    LIFT,
    LIFT_SUD,
}

pub struct Option {
    pub placement_left: Placement,
    pub placement_right: Placement,
    pub gauge: Gauge,
    pub assist: Assist,
    pub display_area: DisplayArea,
    pub flip: bool,
}

impl Option {
    pub fn new() -> Option {
        return Option {
            placement_left: Placement::NORMAL,
            placement_right: Placement::NORMAL,
            gauge: Gauge::NORMAL,
            assist: Assist::OFF,
            display_area: DisplayArea::OFF,
            flip: false,
        };
    }
}

pub struct NoteJudge {
    pub lane: i32,
    pub num: u64,
    pub ms: u64,
    pub judge: Judge,
    pub diff: f64,
}

impl NoteJudge {
    pub fn new() -> NoteJudge {
        return NoteJudge {
            lane: 0,
            num: 0,
            ms: 0,
            judge: JUDGE_YET,
            diff: 0.0,
        };
    }
}

#[allow(non_camel_case_types)]
pub enum Lamp {
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
    pub fn new() -> Result {
        return Result {
            pgreat_early: 0,
            great_early: 0,
            good_early: 0,
            bad_early: 0,
            poor_early: 0,
            kpoor_early: 0,
            pgreat_late: 0,
            great_late: 0,
            good_late: 0,
            bad_late: 0,
            poor_late: 0,
            kpoor_late: 0,
            combo_break: 0,
            miss_count: -1,
            max_combo: 0,
            now_combo: 0,
            note_judges: Vec::new(),
        };
    }

    pub fn push(&mut self, note_judge: NoteJudge) {
        match note_judge.judge {
            PGREAT_EARLY => {
                self.pgreat_early += 1;
            }
            GREAT_EARLY => {
                self.great_early += 1;
            }
            GOOD_EARLY => {
                self.good_early += 1;
            }
            BAD_EARLY => {
                self.bad_early += 1;
                self.combo_break += 1;
                self.miss_count += 1;
            }
            POOR_EARLY => {
                self.poor_early += 1;
                self.combo_break += 1;
                self.miss_count += 1;
            }
            KPOOR_EARLY => {
                self.kpoor_early += 1;
                self.miss_count += 1;
            }
            PGREAT_LATE => {
                self.pgreat_late += 1;
            }
            GREAT_LATE => {
                self.great_late += 1;
            }
            GOOD_LATE => {
                self.good_late += 1;
            }
            BAD_LATE => {
                self.bad_late += 1;
                self.combo_break += 1;
                self.miss_count += 1;
            }
            POOR_LATE => {
                self.poor_late += 1;
                self.combo_break += 1;
                self.miss_count += 1;
            }
            KPOOR_LATE => {
                self.kpoor_late += 1;
                self.miss_count += 1;
            }
            _ => {}
        };
        self.note_judges.push(note_judge);
    }
}

pub struct PlayState {
    pub chart: Chart,
    pub option: Option,
    pub result: Result,
    pub start_ms: u64,
    pub play_ms: u64,
    pub last_play_ms: u64,
    pub now_pulse: u64,
    pub last_pulse: u64,
    pub lane_timelines: [Vec<usize>; MAX_LANE],
}

impl PlayState {
    pub fn new() -> PlayState {
        return PlayState {
            chart: Chart::new(),
            option: Option::new(),
            result: Result::new(),
            start_ms: 0,
            play_ms: 0,
            last_play_ms: 0,
            now_pulse: 0,
            last_pulse: 0,
            lane_timelines: Default::default(),
        };
    }

    pub fn init(&mut self, chart: Chart) {
        // Initialize the play state
        self.chart = chart;
        self.result = Result::new();
        self.start_ms = 0;
        self.play_ms = 0;
        self.last_play_ms = 0;
        self.now_pulse = 0;
        self.last_pulse = 0;
        self.lane_timelines = Default::default();
    }

    pub fn update(&mut self, time_manager: &TimeManager) -> State {
        // Update the play state
        self.last_play_ms = self.play_ms;
        self.play_ms = time_manager.elapsed().as_millis() as u64 - self.start_ms;
        self.last_pulse = self.now_pulse;
        self.now_pulse = self.chart.ms_to_pulse(self.play_ms);

        if matches!(self.option.assist, Assist::AUTO_PLAY) || true {
            let mut note_tmp = Note::new();
            note_tmp.pulse = self.last_pulse;
            for note in self
                .chart
                .notes
                .iter_mut()
                .filter(|note| note.pulse < note_tmp.pulse)
            {
                if self.now_pulse < note.pulse {
                    break;
                }
                // wav_manager.play(&note);
                if note.num != 0 {
                    self.result.push(NoteJudge {
                        lane: note.lane,
                        num: note.num,
                        ms: self.play_ms,
                        judge: PGREAT_EARLY,
                        diff: 0.0,
                    });
                }
            }
        } else {
            // normal play
        }
        State::Continue
    }

    pub fn set_lane_timelines(&mut self) {
        // Set the lane timelines
        for lane_timeline in self.lane_timelines.iter_mut() {
            lane_timeline.clear();
        }

        let placements = match self.chart.info.mode {
            BEAT_5K => vec![1, 2, 3, 4, 5],
            BEAT_7K => vec![1, 2, 3, 4, 5, 6, 7],
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
