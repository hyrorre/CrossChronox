//
//  ScoreData.hpp
//  CrossChronox
//
//  Created by HY_RORRE on 2016/10/10.
//  Copyright © 2016年 hyrorre. All rights reserved.
//

#ifndef ScoreData_hpp
#define ScoreData_hpp

#include "pch.hpp"
#include "TimeManager.hpp"
#include "WavBuffer.hpp"
#include "ScoreInfoBase.hpp"

// Define class ScoreData based on bmson specs
// http://bmson-spec.readthedocs.io/en/master/doc/index.html

using pulse_t = unsigned long;
using event_id_t = unsigned long;

// bar-line event
struct BarLine{
	pulse_t y; // pulse number
	BarLine(pulse_t y): y(y){}
};

enum Judge{
	YET = -1,
	PGREAT = 0,
	GREAT,
	GOOD,
	BAD,
	POOR,
	
	MAX
};

// sound note
struct Note{
	using lane_t = int;
	lane_t x;          // CrossChronox supports only beat and popn(that have integer lane)
	pulse_t y;      // pulse number
	pulse_t l;      // length (0: normal note; greater than zero (length in pulses): long note)
	size_t num = 0; // playable note count (0から始まる)
	WavBuffer* wavbuf_ptr = nullptr;
	Judge judge = Judge::YET;
	
	ms_type ms;     // time(ms) that the note should be handled.
    
	Note(){}
	//Note(int x, pulse_t y, pulse_t l): x(x), y(y), l(l){}
	Note(lane_t x, pulse_t y, pulse_t l, size_t num, WavBuffer* p = nullptr): x(x), y(y), l(l), num(num), wavbuf_ptr(p){}
    bool operator< (const Note& other) const{
        return y < other.y;
    }
};

// bpm note
struct BpmEvent{
	pulse_t y = 0;  // pulse number
	double bpm = 0; // bpm
	pulse_t duration = 0; // pulses from the event to switch next bpm
	BpmEvent(){}
	BpmEvent(pulse_t y, double bpm = 0, pulse_t duration = 0): y(y), bpm(bpm), duration(duration){}
	virtual ~BpmEvent(){}
	bool operator<(const BpmEvent& other) const{
		return other > *this;
	}
	bool operator>(const BpmEvent& other) const{
		return y > other.y;
	}
	ms_type NextEventMs(pulse_t pulse, pulse_t resolution) const{
		return MinToMs((pulse - y) / (bpm * resolution));
	}
};

// stop note
struct StopEvent : public BpmEvent{
	StopEvent(){}
	StopEvent(pulse_t y, pulse_t d): BpmEvent(y, 0, d){}
	bool operator<(const BpmEvent& other) const{
		return y <= other.y;
	}
	bool operator>(const BpmEvent& other) const{
		return y > other.y;
	}
	ms_type NextEventMs(pulse_t pulse, pulse_t resolution) const{
		return MinToMs((pulse + duration - y) / (bpm * resolution));
	}
};

// picture file
struct BGAHeader{
	event_id_t id;      // self-explanatory
	std::string name;   // picture file name
};

// bga note
struct BGAEvent{
	pulse_t y;	    // pulse number
	event_id_t id;	// corresponds to BGAHeader.id
};

// bga
struct BGA{
	std::vector<BGAHeader> bga_header;   // picture id and filename
	std::vector<BGAEvent>  bga_events;   // picture sequence
	std::vector<BGAEvent>  layer_events; // picture sequence overlays bga_notes
	std::vector<BGAEvent>  poor_events;  // picture sequence when missed
};

const bool TOTAL_RELATIVE = false;
const bool TOTAL_ABSOLUTE = true;

enum Mode{
	MODE_BEAT_5K,
	MODE_BEAT_7K,
	MODE_BEAT_10K,
	MODE_BEAT_14K,
	MODE_POPN_5K,
	MODE_POPN_9K,
};

struct ScoreInfo : public ScoreInfoBase{
	using judge_ms_type = std::array<int,3>;
	
	std::string   title;                 // self-explanatory
	std::string   subtitle;              // self-explanatory
	std::string   artist;                // self-explanatory
	std::vector<std::string> subartists; // ["key:value"]
	std::string   genre;                 // self-explanatory
	//std::string   mode_hint = "beat-7k"; // layout hints, e.g. "beat-7k", "popn-5k", "generic-nkeys"
	Mode          mode;
	std::string   chart_name;            // e.g. "HYPER", "FOUR DIMENSIONS"
	int           difficulty = 0;
	size_t        level = 0;             // self-explanatory
	double        init_bpm = 130;        // self-explanatory
	//double        judge_rank = 100;      // relative judge width
	judge_ms_type judge_ms;
	bool          total_type = TOTAL_RELATIVE;
	double        total = 100;           // relative or absolute lifebar gain
	std::string   back_image;            // background image filename
	std::string   eyecatch_image;        // eyecatch image filename
	std::string   banner_image;          // banner image filename
	std::string   preview_music;         // preview music filename
	pulse_t resolution = 240;            // pulses per quarter note
	pulse_t end_y = 0;
	
	double max_bpm;                      // calc from the data
	double min_bpm;                      // calc from the data
	double base_bpm = 0;                 // calc from the data
	size_t note_count = 0;               // calc from the data
	std::string   md5;                   // use to identify score in level table and IR
	//int peek_vol;                        // use it if replaygain is implemented
	bool random_flag = false;            // if #RANDOM is used, it should not be registered IR
	
	std::string GetTitleSubtitle() const{
		return title + ' ' + subtitle;
	}
	
	ScoreInfo(){}
	ScoreInfo(fs::path path): ScoreInfoBase(path){}
};

struct ScoreData : boost::noncopyable{
	std::string                 version;        // bmson version
	ScoreInfo                   info;           // information, e.g. title, artist, …
	std::vector<BarLine>        lines;          // location of bar-lines in pulses
    std::vector<std::unique_ptr<BpmEvent>> bpm_events;     // bpm events and stop events
	//std::vector<SoundChannel>   sound_channels; // note data
	BGA                         bga;            // bga data
	
    std::vector<std::unique_ptr<Note>> notes; // Note timeline
    std::vector<std::unique_ptr<WavBuffer>> wavbufs;
	
	pulse_t MsToPulse(ms_type ms) const;
	//ms_type PulseToMs(pulse_t pulse) const;
	
	void Init(){
		version.clear();
		info = ScoreInfo();
		lines.clear();
		bpm_events.clear();
		bga = BGA();
		notes.clear();
		wavbufs.clear();
	}
};

#endif /* ScoreData_hpp */
