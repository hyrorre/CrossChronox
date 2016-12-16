//
//  WavManager.cpp
//  CrossChronox
//
//  Created by HY_RORRE on 2016/11/28.
//  Copyright © 2016年 hyrorre. All rights reserved.
//

#include "WavManager.hpp"

void WavManager::Update(){
	auto l = [](std::unique_ptr<WavPlayer>& p){ return p->GetStatus() == sf::SoundSource::Status::Stopped; };
	player.remove_if(l);
}

void WavManager::PlayWav(const Note* note){
	if(note && note->wavbuf_ptr){
		player.emplace_front(new WavPlayer(note));
	}
}
