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
#include "ScoreData.hpp"
#include "WavManager.hpp"
#include "Result.hpp"

class Application;

class ScorePlayer{
	WavManager wav_manager;
	ScoreData score;
	Result result;
	
	static ms_type start_ms;

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
