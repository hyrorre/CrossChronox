#pragma once

#include "pch.hpp"
#include "ScoreInfo.hpp"
#include "System/TimeManager.hpp"
#include "WavBuffer.hpp"

// Define class ScoreData based on bmson specs
// http://bmson-spec.readthedocs.io/en/master/doc/index.html

using event_id_t = unsigned long;

// bar-line event
struct BarLine {
    pulse_t pulse; // pulse number
    BarLine(pulse_t pulse) : pulse(pulse) {}
};

inline bool IsComboContinuous(Judge judge) {
    return (Judge::PGREAT <= judge && judge <= Judge::GOOD);
}

// sound note
using lane_t = int;
enum { MAX_LANE = 20 };

struct Note {
    lane_t lane, lane_origin; // CrossChronox supports only beat and popn(that have integer lane)
    pulse_t pulse;            // pulse number
    pulse_t len;              // length (0: normal note; greater than zero (length in pulses): long note)
    size_t num = 0;           // playable note count (1から始まる)
    WavBuffer* wavbuf_ptr = nullptr;
    Judge judge = JUDGE_YET;
    int empty_poor_count = 0;

    ms_type ms; // time(ms) that the note should be handled.
    ms_type lnend_ms = 0;

    Note() {}
    // Note(int x, pulse_t y, pulse_t l): x(x), y(y), l(l){}
    Note(lane_t lane, pulse_t pulse, pulse_t len, size_t num, WavBuffer* p = nullptr) : lane(lane), lane_origin(lane), pulse(pulse), len(len), num(num), wavbuf_ptr(p) {}
    bool operator<(const Note& other) const {
        return pulse < other.pulse;
    }
};

// bpm note
struct BpmEvent {
    pulse_t pulse = 0;    // pulse number
    double bpm = 0;       // bpm
    pulse_t duration = 0; // pulses from the event to switch next bpm
    ms_type ms = 0;
    BpmEvent() {}
    BpmEvent(pulse_t pulse, double bpm = 0, pulse_t duration = 0) : pulse(pulse), bpm(bpm), duration(duration) {}
    virtual ~BpmEvent() {}
    bool operator<(const BpmEvent& other) const {
        return other > *this;
    }
    bool operator>(const BpmEvent& other) const {
        return pulse > other.pulse;
    }
    virtual ms_type NextEventMs(pulse_t pulse, pulse_t resolution) const {
        return ms + MinToMs((pulse - this->pulse) / (bpm * resolution));
    }
};

// stop note
struct StopEvent : public BpmEvent {
    StopEvent() {}
    StopEvent(pulse_t pulse, pulse_t d) : BpmEvent(pulse, 0, d) {}
    bool operator<(const BpmEvent& other) const {
        return pulse <= other.pulse;
    }
    bool operator>(const BpmEvent& other) const {
        return pulse > other.pulse;
    }
    ms_type NextEventMs(pulse_t pulse, pulse_t resolution) const {
        return ms + MinToMs((pulse + duration - this->pulse) / (bpm * resolution));
    }
};

// picture file
struct BGAHeader {
    event_id_t id;    // self-explanatory
    std::string name; // picture file name
};

// bga note
struct BGAEvent {
    pulse_t pulse; // pulse number
    event_id_t id; // corresponds to BGAHeader.id
};

// bga
struct BGA {
    std::vector<BGAHeader> bga_header;  // picture id and filename
    std::vector<BGAEvent> bga_events;   // picture sequence
    std::vector<BGAEvent> layer_events; // picture sequence overlays bga_notes
    std::vector<BGAEvent> poor_events;  // picture sequence when missed
};

struct ScoreData : private boost::noncopyable {
    std::string version;                               // bmson version
    ScoreInfo info;                                    // information, e.g. title, artist, …
    std::vector<BarLine> lines;                        // location of bar-lines in pulses
    std::vector<std::unique_ptr<BpmEvent>> bpm_events; // bpm events and stop events
    // std::vector<SoundChannel>   sound_channels; // note data
    BGA bga; // bga data

    std::vector<std::unique_ptr<Note>> notes; // Note timeline
    std::vector<std::unique_ptr<WavBuffer>> wavbufs;

    pulse_t MsToPulse(ms_type ms) const;
    // ms_type PulseToMs(pulse_t pulse) const;

    void Init() {
        version.clear();
        info = ScoreInfo();
        lines.clear();
        bpm_events.clear();
        bga = BGA();
        notes.clear();
        wavbufs.clear();
    }
};
