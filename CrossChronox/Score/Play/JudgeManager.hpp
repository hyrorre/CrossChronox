//
//  JudgeManager.hpp
//  CrossChronox
//
//  Created by HY_RORRE on 2017/01/19.
//  Copyright © 2017年 hyrorre. All rights reserved.
//

#ifndef JudgeManager_hpp
#define JudgeManager_hpp

#include "pch.hpp"
#include "ScoreData.hpp"

//enum{
//	JUDGE_ALGORITHM_LR2,
//	JUDGE_ALGORITHM_IIDX,
//	JUDGE_ALGORITHM_LOWEST_NOTE,
//	
//	MAX_JUDGE_ALGORITHM
//};

namespace JudgeManager{
	struct NoteJudge{
		Note* note = nullptr;
		Judge judge = JUDGE_YET;
		bool cb_flag = false;
		
		NoteJudge(){}
		NoteJudge(Note* note, Judge judge, bool cb_flag): note(note), judge(judge), cb_flag(cb_flag){}
	};
	
	bool GetPushNoteJudge(std::vector<Note*>& lane_timeline, ms_type push_ms, lane_t lane, Mode mode, LnType ln_type, NoteJudge* out);
	
	bool GetReleaseNoteJudge(std::vector<Note*>& lane_timeline, ms_type push_ms, lane_t lane, Mode mode, LnType ln_type, NoteJudge* out);
}

#endif /* JudgeManager_hpp */
