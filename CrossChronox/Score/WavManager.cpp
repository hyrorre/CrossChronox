//
//  WavManager.cpp
//  CrossChronox
//
//  Created by HY_RORRE on 2016/11/28.
//  Copyright © 2016年 hyrorre. All rights reserved.
//

#include "WavManager.hpp"

void WavManager::Update(){
	
}

void WavManager::PlayWav(const Note* note){
	if(note && note->wavbuf_ptr){
		for(auto& player : players){
			if(player.GetStatus() == sf::Sound::Status::Stopped){
				player.ResetSound(note);
				break;
			}
		}
	}
}
