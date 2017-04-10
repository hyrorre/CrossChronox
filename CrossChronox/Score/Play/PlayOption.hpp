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
	PLACE_NORMAL,
	RANDOM,
	S_RAN,
	H_RAN,
	R_RAN,
	MIRROR,
	
	SYNC_RAN,
	SYM_RAN
};

enum{
	GAUGE_NORMAL,
	A_EASY,
	EASY,
	HARD,
	EX_HARD,
	HARARD
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

enum{
	ASSIST_OFF,
	A_SCRATCH,
	FIVE_KEYS,
	LEGACY,
	A_SCR_5KEYS,
	A_SCR_LEGACY,
	LEGACY_5KEYS,
	FULL_ASSIST,
	
	AUTO_PLAY
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
	LIFT_SUD
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
	int note_display_time = 350; // 緑数字
	double hs = 1.0;
	
public:
	int GetHsType() const;
	int GetSudHidPos() const;
	int GetNoteDisplayTime() const;
	double GetHs() const;
	
	void SetHsType(int value);
	void SetSudHidPos(int value);
	void SetNoteDisplayTime(int value);
	void SetHs(double value);
	
	void IncrHs(double bpm);
	void DecrHs(double bpm);
	double GetHsBpm(double bpm) const;
	void SetHsBpm(double bpm, double value);
};

class PlayOption{
	std::array<int, MAX_SIDE> placements = {{PLACE_NORMAL, PLACE_NORMAL}};
	int gauge_type = GAUGE_NORMAL;
	int assist_type = ASSIST_OFF;
	int display_area = SUD_HID_OFF;
	bool flip = 0;
	
public:
	HsOption hs_option;
	
	int GetPlacement(Side side) const{
		return placements[side];
	}
	
	void SetPlacement(Side side, int value){
		placements[side] = value;
	}
	
	int GetGaugeType() const{
		return gauge_type;
	}
	
	void SetGaugeType(int value){
		gauge_type = value;
	}
	
	int GetAssistType() const{
		return assist_type;
	}
	
	void SetAssistType(int value){
		assist_type = value;
	}
	
	int GetDisplayArea() const{
		return display_area;
	}
	
	void SetDisplayArea(int value){
		display_area = value;
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
			result += GetPlacement(static_cast<Side>(i));
		}
		result += (GetAssistType() == AUTO_PLAY);
		return result;
	}
};

#endif /* PlayOption_hpp */
