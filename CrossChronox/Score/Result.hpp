//
//  Result.hpp
//  CrossChronox
//
//  Created by HY_RORRE on 2016/12/18.
//  Copyright © 2016年 hyrorre. All rights reserved.
//

#ifndef Result_hpp
#define Result_hpp

#include "pch.hpp"
#include "ScoreData.hpp"

class Result{
	std::array<size_t, Judge::MAX> judge = {0};
	size_t cb = 0;
	size_t ex_score = 0;
	size_t max_combo = 0;
	size_t now_combo = 0;
	
public:
	size_t GetJudge(Judge j) const{
		return judge[static_cast<int>(j)];
	}
	void SetJudge(Judge j, size_t value){
		judge[static_cast<int>(j)] = value;
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
};


#endif /* Result_hpp */
