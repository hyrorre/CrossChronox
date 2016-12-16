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

class Application;

class ScorePlayer{
	friend Application;
	ScoreData score;
	WavManager wav_manager;
	static ms_type start_ms;
public:
	ScorePlayer(){}
	//void SetScore(ScoreData& score);
	bool Start();
	bool Update();
};

#endif /* ScorePlayer_hpp */
