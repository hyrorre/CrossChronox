//
//  PlayScore.cpp
//  CrossChronox
//
//  Created by HY_RORRE on 2016/10/10.
//  Copyright © 2016年 hyrorre. All rights reserved.
//

#include "PlayScore.hpp"

PlayScore scene_play_score;

PlayScore* scene_play_score_ptr = &scene_play_score;

void PlayScore::Init(){
	for(auto& player : players){
		player.Init();
	}
	ScorePlayer::Start();
}

Scene* PlayScore::Update(){
	int continue_flag = 0;
	for(auto& player : players){
		continue_flag += player.Update();
	}
	if(continue_flag) return scene_play_score_ptr;
	else return nullptr;
}

void PlayScore::Draw() const{
	
}
