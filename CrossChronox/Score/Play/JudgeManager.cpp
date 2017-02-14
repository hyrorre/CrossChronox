//
//  JudgeManager.cpp
//  CrossChronox
//
//  Created by HY_RORRE on 2017/01/19.
//  Copyright © 2017年 hyrorre. All rights reserved.
//

#include "JudgeManager.hpp"

namespace JudgeManager{
	//enum{ MAX_JUDGE_TABLE_VALUE = 6 };
	
	
	// BMSの各判定の範囲(+-ms)。PGREAT, GREAT, GOOD, BAD, MISS空POORの順
	const ms_type bms_judge_table[] = { 20, 60, 160, 250, 1000 };
	
	// BMSのLN終端の各判定の範囲(+-ms)。PGREAT, GREAT, GOOD, BAD, MISS空POORの順
	const ms_type bms_lnend_judge_table[] = { 100, 150, 200, 250, 1000 };
	
	// BMSのスクラッチの各判定の範囲(+-ms)。PGREAT, GREAT, GOOD, BAD, MISS空POORの順
	const ms_type scr_judge_table[] = { 30, 75, 200, 300, 1000 };
	
	// BMSのBSS終端の各判定の範囲(+-ms)。PGREAT, GREAT, GOOD, BAD, MISS空POORの順
	const ms_type bssend_judge_table[] = { 100, 150, 250, 300, 1000 };
	
	// PMSの各判定の範囲(+-ms)。PGREAT, GREAT, GOOD, BAD, MISS空POORの順
	const ms_type pms_judge_table[] = { 25, 75, 175, 200, 1000 };
	
	// PMSのLN終端の各判定の範囲(+-ms)。PGREAT, GREAT, GOOD, BAD, MISS空POORの順
	const ms_type pms_lnend_judge_table[] = { 100, 150, 175, 200, 1000 };
	
	const ms_type* GetJudgeTable(lane_t lane, bool lnend_flag, Mode mode){
		if(mode <= BEAT_14K){ // if bms
			if(lane == 8){ // if scratch
				return (lnend_flag ? bssend_judge_table : scr_judge_table);
			}
			// if normal note
			else return (lnend_flag ? bms_lnend_judge_table : bms_judge_table);
		}
		// if pms
		else return (lnend_flag ? pms_lnend_judge_table : pms_judge_table);
	}
	
	bool GetPushNoteJudge(const std::vector<Note*>& lane_timeline, ms_type push_ms, lane_t lane, Mode mode, LnType ln_type, std::vector<NoteJudge>* out){
		out->clear(); // initialize out
		if(!lane_timeline.empty()){
			const ms_type* judge_table = GetJudgeTable(lane_timeline.back()->lane, false, mode);
			for(Note* note : lane_timeline){
				if(note->judge == JUDGE_YET) continue;
				int diff_ms = note->ms - push_ms; // ms_type is not appropriate becase it is unsigned
				if(judge_table[BAD] < diff_ms) break;
				if(judge_table[POOR] < -diff_ms){
					out->emplace_back(note, POOR, false);
					continue;
				}
				diff_ms = abs(diff_ms);
				for(Judge judge = PGREAT; judge <= BAD; judge = Judge(judge + 1)){
					if(judge_table[judge] < diff_ms){
						out->emplace_back(note, judge, (judge == BAD));
						return true;
					}
				}
			}
		}
		return false;
	}
	
	bool GetReleaseNoteJudge(std::vector<Note*>& lane_timeline, ms_type release_ms, lane_t lane, Mode mode, LnType ln_type, NoteJudge* out){
		
	}
}
