﻿#pragma once

#include "pch.hpp"
#include "Score/ScoreData/ScoreData.hpp"
#include "System/TimeManager.hpp"
#include "PlayOption.hpp"

struct JudgeInfo{
	ms_type ms = 0;
	Judge judge = JUDGE_YET;
	size_t combo = 0;
	
	JudgeInfo(){}
	JudgeInfo(ms_type ms, Judge judge, size_t combo): ms(ms), judge(judge), combo(combo){}
};

class Result{
	std::array<size_t, MAX_JUDGE> judge_counts = {{0}};
	size_t cb = 0;
	size_t ex_score = 0;
	size_t max_combo = 0;
	size_t now_combo = 0;
	
	std::array<std::vector<JudgeInfo>, 2> timelines;
	
	size_t JudgeToExScore(Judge j){
		if(j == PGREAT) return 2;
		if(j == GREAT) return 1;
		return 0;
	};
	
public:
	size_t GetJudgeCount(Judge j) const{
		return judge_counts[static_cast<size_t>(j)];
	}
	void SetJudgeCount(Judge j, size_t value){
		judge_counts[static_cast<size_t>(j)] = value;
	}
	size_t GetComboBreak() const{
		return cb;
	}
	void SetComboBreak(size_t value){
		cb = value;
	}
	size_t GetExScore() const{
		return ex_score;
	}
	void SetExScore(size_t value){
		ex_score = value;
	}
	size_t GetMaxCombo() const{
		return max_combo;
	}
	void SetMaxCombo(size_t value){
		max_combo = value;
	}
	size_t GetNowCombo() const{
		return now_combo;
	}
	void SetNowCombo(size_t value){
		now_combo = value;
	}
	void Push(Side side, ms_type ms, Judge judge, bool combo_break){
		if(PGREAT <= judge){
			size_t combo = GetNowCombo();
			if(IsComboContinuous(judge)){
				SetNowCombo(++combo);
				SetMaxCombo(std::max(GetNowCombo(), GetMaxCombo()));
				SetExScore(GetExScore() + JudgeToExScore(judge));
			}
			else if(combo_break){
				SetNowCombo(0);
				SetComboBreak(GetComboBreak() + 1);
			}
			SetJudgeCount(judge, GetJudgeCount(judge) + 1);
			timelines[side].emplace_back(ms, judge, combo);
		}
	}
	JudgeInfo GetLastJudgeInfo(Side side) const{
		auto& tl = timelines[side];
		if(tl.empty()) return JudgeInfo();
		else return tl.back();
	}
	std::wstring GetResultStr() const;
};

