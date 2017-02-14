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
#include "Score/ScoreData/ScoreData.hpp"
#include "System/Input/InputManager.hpp"

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
	
	bool UpdateLane(const std::vector<Note*>& lane_timeline, InputManager::KeyState key_state, ms_type now_ms, lane_t lane, Mode mode, LnType ln_type, std::vector<NoteJudge>* out);
}

#endif /* JudgeManager_hpp */
