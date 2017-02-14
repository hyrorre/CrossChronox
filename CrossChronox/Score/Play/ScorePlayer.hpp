//
//  ScorePlayer.hpp
//  CrossChronox
//
//  Created by HY_RORRE on 2016/11/04.
//  Copyright © 2016年 hyrorre. All rights reserved.
//

#ifndef ScorePlayer_hpp
#define ScorePlayer_hpp

#include "pch.hpp"
#include "Score/ScoreData/ScoreData.hpp"
#include "WavManager.hpp"
#include "Score/ScoreData/Result.hpp"

class Application;

class ScorePlayer{
	WavManager wav_manager;
	ScoreData score;
	Result result;
	
	static ms_type start_ms;
	
	std::vector<std::vector<Note*>> lane_timelines = std::vector<std::vector<Note*>>(MAX_LANE);
	void SetLaneTimelines();
	
	void Judge();
	
public:
	enum State{
		FINISH,
		CONTINUE
	};
	ScorePlayer(){}
	void Init();
	void Clear(){
		wav_manager.Clear();
		score.Init();
	}
	static ms_type GetPlayMs(){
		return now_ms - start_ms;
	}
	const ScoreData& GetScore() const{
		return score;
	}
	const Result& GetResult() const{
		return result;
	}
	static void Start();
	State Update();
};

#endif /* ScorePlayer_hpp */
