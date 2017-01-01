//
//  ScoreInfo.hpp
//  CrossChronox
//
//  Created by HY_RORRE on 2016/12/30.
//  Copyright © 2016年 hyrorre. All rights reserved.
//

#ifndef ScoreInfo_hpp
#define ScoreInfo_hpp

#include "pch.hpp"
#include "ScoreInfoBase.hpp"

using pulse_t = unsigned long;

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
	
	std::wstring   title;                 // self-explanatory
	std::wstring   subtitle;              // self-explanatory
	std::wstring   artist;                // self-explanatory
	std::vector<std::wstring> subartists; // ["key:value"]
	std::wstring   genre;                 // self-explanatory
	//std::string   mode_hint = "beat-7k"; // layout hints, e.g. "beat-7k", "popn-5k", "generic-nkeys"
	Mode          mode;
	std::wstring   chart_name;            // e.g. "HYPER", "FOUR DIMENSIONS"
	int           difficulty = 0;
	size_t        level = 0;             // self-explanatory
	double        init_bpm = 130;        // self-explanatory
	//double        judge_rank = 100;      // relative judge width
	judge_ms_type judge_ms;
	bool          total_type = TOTAL_RELATIVE;
	double        total = 100;           // relative or absolute lifebar gain
	std::wstring   back_image;            // background image filename
	std::wstring   eyecatch_image;        // eyecatch image filename
	std::wstring   banner_image;          // banner image filename
	std::wstring   preview_music;         // preview music filename
	pulse_t resolution = 240;            // pulses per quarter note
	pulse_t end_y = 0;
	
	double max_bpm;                      // calc from the data
	double min_bpm;                      // calc from the data
	double base_bpm = 0;                 // calc from the data
	size_t note_count = 0;               // calc from the data
	std::string   md5;                   // use to identify score in level table and IR
	//int peek_vol;                        // use it if replaygain is implemented
	bool random_flag = false;            // if #RANDOM is used, it should not be registered IR
	
	std::wstring GetTitleSubtitle() const{
		std::wstringstream ss;
		ss << level << ' ' << title << ' ' << subtitle;
		return ss.str();
	}
	
	ScoreInfo(){}
	ScoreInfo(fs::wpath path): ScoreInfoBase(path){}
	
	template<class Archive>
	void serialize__(Archive& ar, unsigned int ver){
	}
	
private: // ここがシリアライズ処理の実装
	BOOST_SERIALIZATION_SPLIT_MEMBER();
	friend class boost::serialization::access;
	template<class Archive>
	void save(Archive& ar, const unsigned int version) const{
		ar & BOOST_SERIALIZATION_BASE_OBJECT_NVP(ScoreInfoBase);
		ar & boost::serialization::make_nvp("title", title);
		ar & boost::serialization::make_nvp("subtitle", subtitle);
		ar & boost::serialization::make_nvp("artist", artist);
		ar & boost::serialization::make_nvp("subartists", subartists);
		ar & boost::serialization::make_nvp("genre", genre);
		ar & boost::serialization::make_nvp("mode", mode);
		ar & boost::serialization::make_nvp("chart_name", chart_name);
		ar & boost::serialization::make_nvp("difficulty", difficulty);
		ar & boost::serialization::make_nvp("level", level);
		ar & boost::serialization::make_nvp("init_bpm", init_bpm);
		ar & boost::serialization::make_nvp("judge_ms", judge_ms);
		ar & boost::serialization::make_nvp("total_type", total_type);
		ar & boost::serialization::make_nvp("total", total);
		ar & boost::serialization::make_nvp("back_image", back_image);
		ar & boost::serialization::make_nvp("eyecatch_image", eyecatch_image);
		ar & boost::serialization::make_nvp("banner_image", banner_image);
		ar & boost::serialization::make_nvp("preview_music", preview_music);
		ar & boost::serialization::make_nvp("resolution", resolution);
		ar & boost::serialization::make_nvp("end_y", end_y);
		ar & boost::serialization::make_nvp("max_bpm", max_bpm);
		ar & boost::serialization::make_nvp("min_bpm", min_bpm);
		ar & boost::serialization::make_nvp("base_bpm", base_bpm);
		ar & boost::serialization::make_nvp("note_count", note_count);
		ar & boost::serialization::make_nvp("md5", md5);
		ar & boost::serialization::make_nvp("random_flag", random_flag);
	}
	
	template<class Archive>
	void load(Archive& ar, const unsigned int version){
		ar & BOOST_SERIALIZATION_BASE_OBJECT_NVP(ScoreInfoBase);
		std::string tmp;
		std::wstring_convert<std::codecvt_utf8<wchar_t>,wchar_t> cv;
#define MAKE_WSTRING_NVP(x) \
	boost::serialization::make_nvp(#x, tmp); \
	x = cv.from_bytes(tmp)
		
		ar & MAKE_WSTRING_NVP(title);
		ar & MAKE_WSTRING_NVP(subtitle);
		ar & MAKE_WSTRING_NVP(artist);
		std::vector<std::string> tmpvec;
		ar & boost::serialization::make_nvp("subartists", tmpvec);
		subartists.clear();
		for(int i = 0; i < tmpvec.size(); ++i){
			subartists.emplace_back(cv.from_bytes(tmpvec[i]));
		}
		ar & MAKE_WSTRING_NVP(genre);
		
		ar & boost::serialization::make_nvp("mode", mode);
		ar & MAKE_WSTRING_NVP(chart_name);
		ar & boost::serialization::make_nvp("difficulty", difficulty);
		ar & boost::serialization::make_nvp("level", level);
		ar & boost::serialization::make_nvp("init_bpm", init_bpm);
		ar & boost::serialization::make_nvp("judge_ms", judge_ms);
		ar & boost::serialization::make_nvp("total_type", total_type);
		ar & boost::serialization::make_nvp("total", total);
		ar & MAKE_WSTRING_NVP(back_image);
		ar & MAKE_WSTRING_NVP(eyecatch_image);
		ar & MAKE_WSTRING_NVP(banner_image);
		ar & MAKE_WSTRING_NVP(preview_music);
		ar & boost::serialization::make_nvp("resolution", resolution);
		ar & boost::serialization::make_nvp("end_y", end_y);
		ar & boost::serialization::make_nvp("max_bpm", max_bpm);
		ar & boost::serialization::make_nvp("min_bpm", min_bpm);
		ar & boost::serialization::make_nvp("base_bpm", base_bpm);
		ar & boost::serialization::make_nvp("note_count", note_count);
		ar & boost::serialization::make_nvp("md5", md5);
		ar & boost::serialization::make_nvp("random_flag", random_flag);
	}
};

//BOOST_CLASS_EXPORT_GUID(ScoreInfo, "ScoreInfo");
BOOST_CLASS_VERSION(ScoreInfo, 1);

#endif /* ScoreInfo_hpp */
