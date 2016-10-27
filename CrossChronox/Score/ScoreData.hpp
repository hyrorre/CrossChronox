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

//Define class ScoreData based on bmson specs
//http://bmson-spec.readthedocs.io/en/master/doc/index.html

// bar-line event
struct BarLine {
	unsigned long y; // pulse number
};
// sound note
struct Note {
	boost::any x;    // lane
	unsigned long y; // pulse number
	unsigned long l; // length (0: normal note; greater than zero (length in pulses): long note)
	bool c;          // continuation flag
};
// sound channel
struct SoundChannel {
	std::string name; // sound file name
	std::vector<Note> notes;   // notes using this sound
};
// bpm note
struct BpmEvent {
	unsigned long y; // pulse number
	double bpm;      // bpm
};
// stop note
struct StopEvent {
	unsigned long y;        // pulse number
	unsigned long duration; // stop duration (pulses to stop)
};

// picture file
struct BGAHeader {
	unsigned long id; // self-explanatory
	std::string name;   // picture file name
};
// bga note
struct BGAEvent {
	unsigned long y;	// pulse number
	unsigned long id;	// corresponds to BGAHeader.id
};
// bga
struct BGA {
	std::vector<BGAHeader> bga_header;   // picture id and filename
	std::vector<BGAEvent>  bga_events;   // picture sequence
	std::vector<BGAEvent>  layer_events; // picture sequence overlays bga_notes
	std::vector<BGAEvent>  poor_events;  // picture sequence when missed
};

// --- Note type proposal (See comments at notes.x)
//	struct Note {
//		std::string type;  // For a mode that uses multiple note types (such as SOUND VOLTEX) (since JSON cannot have “types”)
//		unsigned long y; // pulse number
//		bool c;          // continuation flag
//	};
//	struct LaneNote: Note {
//		unsigned long x; //??
//		unsigned long l; //length in pulses
//	};

const bool TOTAL_RELATIVE = false;
const bool TOTAL_ABSOLUTE = true;

struct ScoreInfo{
	using judge_ms_type = std::array<int,3>;
	
	std::string   title;                 // self-explanatory
	std::string   subtitle;              // self-explanatory
	std::string   artist;                // self-explanatory
	std::vector<std::string> subartists; // ["key:value"]
	std::string   genre;                 // self-explanatory
	std::string   mode_hint = "beat-7k"; // layout hints, e.g. "beat-7k", "popn-5k", "generic-nkeys"
	std::string   chart_name;            // e.g. "HYPER", "FOUR DIMENSIONS"
	int           difficulty = 0;
	unsigned long level = 0;             // self-explanatory
	double        init_bpm = 130;        // self-explanatory
	//double        judge_rank = 100;      // relative judge width
	judge_ms_type judge_ms;
	bool          total_type = TOTAL_RELATIVE;
	double        total = 100;           // relative or absolute lifebar gain
	std::string   back_image;            // background image filename
	std::string   eyecatch_image;        // eyecatch image filename
	std::string   banner_image;          // banner image filename
	std::string   preview_music;         // preview music filename
	unsigned long resolution = 240;      // pulses per quarter note
	
	double max_bpm;        // calc from the data
	double min_bpm;        // calc from the data
	double base_bpm = 0;   // calc from the data
	unsigned long note_count;            // calc from the data
	std::string   md5;                   // use to identify score in level table and IR
	//int peek_vol;                        //use it if replaygain is implemented
};

struct ScoreData{
	std::string               version;        // bmson version
	ScoreInfo                 info;           // information, e.g. title, artist, …
	std::vector<BarLine>      lines;          // location of bar-lines in pulses
	std::vector<BpmEvent>     bpm_events;     // bpm changes
	std::vector<StopEvent>    stop_events;    // stop events
	std::vector<SoundChannel> sound_channels; // note data
	BGA                       bga;            // bga data
	
	//void Init();
};

#endif /* ScoreData_hpp */
