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
struct BarLine{
	unsigned long y; // pulse number
	BarLine(unsigned long y): y(y){}
};
// sound note
struct Note{
	//boost::any x;    // lane
	int x;           // CrossChronox supports only beat and popn(integer lane)
	unsigned long y; // pulse number
	unsigned long l; // length (0: normal note; greater than zero (length in pulses): long note)
	bool c;          // continuation flag
	unsigned long num = 0;     // playable note count (0から始まる)
	Note(){}
	Note(int x, unsigned long y, unsigned long l, bool c): x(x), y(y), l(l), c(c){}
	Note(int x, unsigned long y, unsigned long l, bool c, unsigned long num): x(x), y(y), l(l), c(c), num(num){}
};
// sound channel
struct SoundChannel{
	std::string name; // sound file name
	std::vector<Note> notes;   // notes using this sound
};
// bpm note
struct BpmEvent{
	unsigned long y; // pulse number
	double bpm;      // bpm
	BpmEvent(){}
	BpmEvent(unsigned long y, double bpm): y(y), bpm(bpm){}
};
// stop note
struct StopEvent{
	unsigned long y;        // pulse number
	unsigned long duration; // stop duration (pulses to stop)
	StopEvent(){}
	StopEvent(unsigned long y, unsigned long d): y(y), duration(d){}
};

// picture file
struct BGAHeader{
	unsigned long id; // self-explanatory
	std::string name;   // picture file name
};
// bga note
struct BGAEvent{
	unsigned long y;	// pulse number
	unsigned long id;	// corresponds to BGAHeader.id
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

struct ScoreInfo{
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
	unsigned long note_count = 0;        // calc from the data
	std::string   md5;                   // use to identify score in level table and IR
	//int peek_vol;                        // use it if replaygain is implemented
	bool random_flag = false;            // if #RANDOM is used, it should not be registered IR
};

struct ScoreData{
	std::string               version;        // bmson version
	ScoreInfo                 info;           // information, e.g. title, artist, …
	std::vector<BarLine>      lines;          // location of bar-lines in pulses
	std::vector<BpmEvent>     bpm_events;     // bpm changes
	std::vector<StopEvent>    stop_events;    // stop events
	std::vector<SoundChannel> sound_channels; // note data
	BGA                       bga;            // bga data
};

#endif /* ScoreData_hpp */
