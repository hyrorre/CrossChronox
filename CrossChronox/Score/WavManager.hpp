//
//  WavManager.hpp
//  CrossChronox
//
//  Created by HY_RORRE on 2016/11/28.
//  Copyright © 2016年 hyrorre. All rights reserved.
//

#ifndef WavManager_hpp
#define WavManager_hpp

#include "pch.hpp"
#include "WavPlayer.hpp"
#include "ScoreData.hpp"

class WavManager{
	static const int MAX_SOUND = 64;
    using container_t = std::vector<WavPlayer>;
	container_t players;
	
public:
	void Update();
	void PlayWav(const Note* note);
	bool Empty() const{
		return players.empty();
	}
	void Init(){
		players.resize(MAX_SOUND);
	}
	void Clear(){
		players.clear();
	}
	
	WavManager(): players(MAX_SOUND){}
};

#endif /* WavManager_hpp */
