//
//  BmsLoader.hpp
//  CrossChronox
//
//  Created by HY_RORRE on 10/7/16.
//  Copyright © 2016 hyrorre. All rights reserved.
//

#ifndef BmsLoader_hpp
#define BmsLoader_hpp

#include "pch.hpp"
#include "ScoreData.hpp"

class BmsLoader: private boost::noncopyable{
	static const int BEAT_RESOLUTION = 240;
	
	struct BarInfo{
		double scale;
		unsigned long start;
		unsigned long length(){
			return scale * BEAT_RESOLUTION * 4;
		}
		BarInfo(double d): scale(d){}
	};
	struct TmpNoteData{
		int bar;
		int channel;
		int index;
		int tmp_bar_pulse;
		int tmp_bar_resolution;
		
		bool lnend = false;
		
		unsigned long bar_pulse = 0;
		
		TmpNoteData(){}
		TmpNoteData(int bar, int channel, int index, int bar_pulse, int bar_resolution): bar(bar), channel(channel), index(index), tmp_bar_pulse(bar_pulse), tmp_bar_resolution(bar_resolution){}
	};
	std::vector<BarInfo> bar_info = std::vector<BarInfo>(1001, BarInfo(1));
	int max_bar = 0;
	boost::ptr_vector<TmpNoteData> tmp_note;
	std::unordered_map<int, double> exbpm;
	std::unordered_map<int, int> stop;
	std::list<int> lnobj;
	ScoreData* out = nullptr;
	const char* nowline = nullptr;
	int line_num = 0; //行番号
	
	int random_num = 0;
	bool parse_nextline_flag = true;
	
	const char* GetArg();
	int GetIndex();
	int GetIndex(const char* str);
	bool ParseLine();
	bool TryParseHeaderLine();
	bool TryParseObjLine();
	bool InitCommands();
	bool Init(ScoreData* out);
public:
	BmsLoader();
	bool Load(const std::string& path, ScoreData* out);
	
	//exceptions
	class LoadError: public std::runtime_error{
	public:
		LoadError(const std::string& msg): std::runtime_error(msg){}
		virtual ~LoadError(){}
	};
	class OpenError: public LoadError{
	public:
		OpenError(const std::string& msg): LoadError(msg){}
		virtual ~OpenError(){}
	};
	class ParseError: public LoadError{
	public:
		ParseError(const std::string& msg): LoadError(msg){}
		virtual ~ParseError(){}
	};
};

#endif /* BmsLoader_hpp */
