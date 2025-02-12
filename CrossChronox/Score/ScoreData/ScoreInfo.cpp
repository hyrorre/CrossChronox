//
//  ScoreInfo.cpp
//  CrossChronox
//
//  Created by HY_RORRE on 2016/12/30.
//  Copyright © 2016年 hyrorre. All rights reserved.
//

#include "ScoreInfo.hpp"


static const std::vector<std::string> mode_str = {
	"beat-5k",
	"beat-7k",
	"beat-10k",
	"beat-14k",
	"popn-5k",
	"popn-9k"
};

const std::string& GetModeString(Mode mode){
	return mode_str[static_cast<size_t>(mode)];
}

std::wstring ScoreInfo::GetInfoStr() const{
//	std::wstringstream ss;
//#define SS(x) ss << #x L": " << x << L'\n'
//	SS(title);
//	SS(subtitle);
//	SS(artist);
//	SS(genre);
//	ss << L"mode" L": " << GetModeString(mode).c_str() << L'\n';
//	SS(chart_name);
//	SS(difficulty);
//	SS(level);
//	SS(note_count);
//	SS(init_bpm);
//	SS(base_bpm);
//	SS(max_bpm);
//	SS(min_bpm);
//	ss << std::endl;
// 
//	return ss.str();
	return L"";
}
