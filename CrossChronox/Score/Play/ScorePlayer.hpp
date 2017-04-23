﻿//
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
#include "Score/Play/Result.hpp"
#include "Score/Play/Account.hpp"

class Application;

class ScorePlayer{
	WavManager wav_manager;
	ScoreData score;
	Result result;
	Account* account = &guest_account;
	
	static ms_type start_ms;
	
	std::vector<std::vector<Note*>> lane_timelines = std::vector<std::vector<Note*>>(MAX_LANE);
	void SetLaneTimelines();
	
	void JudgeAuto();
	void Judge();
	
	Side LaneToSide(lane_t lane){
		return static_cast<Side>(score.info.mode != Mode::POPN_9K && 9 <= lane);
	}
	
	void SetAccount(Account& account){
		this->account = &account;
	}
	
	ms_type play_ms = 0;
	ms_type last_play_ms = 0;
	pulse_t now_pulse = 0;
	pulse_t last_pulse = 0;
	
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
		return (now_ms > start_ms ? now_ms - start_ms : 0);
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
