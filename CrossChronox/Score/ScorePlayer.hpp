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

class ScorePlayer{
	ScoreData* score = nullptr;
	ms_type start_ms;
	std::vector<sf::SoundBuffer> buf;
public:
	ScorePlayer(){}
	ScorePlayer(const ScoreData& score);
	void SetScore(const ScoreData& score);
	bool Start();
	bool Update();
};

#endif /* ScorePlayer_hpp */
