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

class ScorePlayer{
	ScoreData score;
	WavManager wav_manager;
	Result result;
	
	static ms_type start_ms;

public:
	enum State{
		FINISH,
		CONTINUE
	};
	ScorePlayer(){}
	static void Start();
	State Update();
	void Init(){
		score.Init();
		wav_manager.Init();
		result = Result();
	}
};

#endif /* ScorePlayer_hpp */
