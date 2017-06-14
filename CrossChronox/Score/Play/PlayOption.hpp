//
//  PlayOption.hpp
//  CrossChronox
//
//  Created by HY_RORRE on 2017/04/10.
//  Copyright © 2017年 hyrorre. All rights reserved.
//

#ifndef PlayOption_hpp
#define PlayOption_hpp

#include "pch.hpp"

enum{
	PLACE_NORMAL = 0,
	RANDOM = 1,
	R_RAN = 2,
	S_RAN = 3,
	H_RAN = 4,
	MIRROR = 5,
	
	MAX_PLACE_NOBATTLE = 6,
	
	SYNC_RAN = 6,
	SYM_RAN = 7,
	
	MAX_PLACE_BATTLE = 8
};

const char* placement_str[] = {
	"NORMAL",
	"RANDOM",
	"R-RANDOM",
	"S-RANDOM",
	"H-RANDOM",
	"MIRROR",
	"SYNCHRONIZE RANDOM",
	"SYMMETRY RANDOM"
};

enum{
	GAUGE_NORMAL,
	A_EASY,
	EASY,
	HARD,
	EX_HARD,
	HAZARD,
	
	MAX_GAUGE
};

const char* gauge_type_str[] = {
	"NORMAL",
	"ASSISTED EASY",
	"EASY",
	"HARD",
	"EX HARD",
	"HAZARD"
};

enum{
	LAMP_NOPLAY,
	LAMP_FAILED,
	LAMP_ASSIST,
	LAMP_EASY,
	LAMP_NORMAL,
	LAMP_HARD,
	LAMP_EX_HARD,
	LAMP_FC,  // FULL COMBO
	LAMP_PFC, // PERFECT FULL COMBO
	LAMP_MFC, // MAX FULL COMBO
};

const char* lamp_str[] = {
	"NOPLAY",
	"FAILED",
	"ASSIST CLEAR",
	"EASY CLEAR",
	"CLEAR",
	"HARD CLEAR",
	"EX HARD CLEAR",
	"FULLCOMBO",
	"PERFECT FULLCOMBO",
	"MAX FULLCOMBO"
};

enum{
	ASSIST_OFF,
	A_SCRATCH,
	FIVE_KEYS,
	LEGACY,
	A_SCR_5KEYS,
	A_SCR_LEGACY,
	LEGACY_5KEYS,
	FULL_ASSIST,
	
	AUTO_PLAY,
	
	MAX_ASSIST
};

const char* assist_str[] = {
	"OFF",
	"AUTO SCRATCH",
	"5KEYS",
	"LEGACY NOTE",
	"A-SCR & 5KEYS",
	"A-SCR & LEGACY",
	"5KEYS & LEGACY",
	"FULL ASSIST",
	"AUTOPLAY"
};

// BEAT_10K and BEAT_14K use both sides
enum Side{
	LEFT,
	RIGHT,
	MAX_SIDE
};

enum{
	SUD_HID_OFF,
	SUD,
	HID,
	SUD_HID,
	LIFT,
	LIFT_SUD,
	
	MAX_DISPLAY_AREA
};

enum{
	NHS, // Normal Hi-Speed
	FHS, // Floating Hi-Speed
	MAXBPM_FIX,
	MINBPM_FIX,
	BASEBPM_FIX,
};

class HsOption{
	int hs_type = NHS;
	int sud_hid_pos = 10; // 白数字
	int lift_pos = 10; // リフトの白数字
	int note_display_time = 350; // 緑数字
	double hs = 1.0;
	
	double hsbpm_step = 10;
	int sud_hid_step = 10;
	int lift_step = 10;
	
public:
	int GetHsType() const{
		return hs_type;
	}
	int GetLiftPos() const{
		return lift_pos;
	}
	int GetSudHidPos() const{
		return sud_hid_pos;
	}
	int GetNoteDisplayTime() const{
		return note_display_time;
	}
	double GetHs() const{
		return hs;
	}
	
	void SetHsType(int value){
		hs_type = value;
	}
	void SetSudHidPos(int value){
		sud_hid_pos = value;
	}
	void SetLiftPos(int value){
		lift_pos = value;
	}
	void SetNoteDisplayTime(int value){
		note_display_time = value;
	}
	void SetHs(double value){
		hs = value;
	}
	
	void AddHs(double bpm, int num){
		// TODO: hs_typeに応じてSwitch
		// if NHS
		hs += 0.25 * num;
		boost::algorithm::clamp(hs, 1.0, 4.0);
	}
	
	double GetHsBpm(double bpm) const;
	void SetHsBpm(double bpm, double value);
	
	void SetHsBpmStep(double value){
		hsbpm_step = value;
	}
	void SetSudHidStep(int value){
		sud_hid_step = value;
	}
	void SetLiftStep(int value){
		lift_step = value;
	}
};

enum {
	SP,
	DP,
	BATTLE
};

class PlayOption{
	int style = SP;
	std::array<int, MAX_SIDE> placements = {{PLACE_NORMAL, PLACE_NORMAL}};
	int gauge_type = GAUGE_NORMAL;
	int assist_type = ASSIST_OFF;
	int display_area = SUD_HID_OFF;
	bool flip = 0;
	
public:
	HsOption hs_option;
	
	int GetStyle() const{
		return style;
	}
	
	void SetStyle(int value){
		style = value;
	}
	
	int GetPlacement(Side side) const{
		return placements[side];
	}
	
	void SetPlacement(Side side, int value){
		placements[side] = value;
	}
	
	void AddPlacement(Side side, int value){
		int max_placement = (GetStyle() == BATTLE ? MAX_PLACE_BATTLE : MAX_PLACE_NOBATTLE);
		int new_value = (GetPlacement(side) + value + max_placement) % max_placement;
		SetPlacement(side, new_value);
	}
	
	int GetGaugeType() const{
		return gauge_type;
	}
	
	void SetGaugeType(int value){
		gauge_type = value;
	}
	
	void AddGaugeType(int value){
		SetGaugeType((GetGaugeType() + value + MAX_GAUGE) % MAX_GAUGE);
	}
	
	int GetAssistType() const{
		return assist_type;
	}
	
	void SetAssistType(int value){
		assist_type = value;
	}
	
	void AddAssistType(int value){
		SetAssistType((GetAssistType() + value + MAX_ASSIST) % MAX_ASSIST);
	}
	
	int GetDisplayArea() const{
		return display_area;
	}
	
	void SetDisplayArea(int value){
		display_area = value;
	}
	
	void AddDisplayArea(int value){
		SetDisplayArea((GetDisplayArea() + value + MAX_DISPLAY_AREA) % MAX_DISPLAY_AREA);
	}
	
	bool GetFlip() const{
		return flip;
	}
	
	void SetFlip(bool value){
		flip = value;
	}
	
	bool CanSaveScore(){
		int result = 0;
		for(int i = 0; i < MAX_SIDE; ++i){
			result += (GetPlacement(static_cast<Side>(i)) == H_RAN);
		}
		result += (GetAssistType() == AUTO_PLAY);
		return result;
	}
	
	std::string GetInfoStr() const;
};

#endif /* PlayOption_hpp */
