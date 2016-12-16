//
//  WavPlayer.hpp
//  CrossChronox
//
//  Created by HY_RORRE on 2016/12/10.
//  Copyright © 2016年 hyrorre. All rights reserved.
//

#ifndef WavPlayer_hpp
#define WavPlayer_hpp

#include "pch.hpp"
#include "ScoreData.hpp"

class WavPlayer{
    const Note* note_ptr = nullptr;
    sf::Sound sound;

public:
	sf::SoundSource::Status GetStatus() const{
		return sound.getStatus();
	}
	void Pause(){
		sound.pause();
	}
	
	WavPlayer(const Note* note_ptr): note_ptr(note_ptr), sound(note_ptr->wavbuf_ptr->buf){
		sound.play();
	}
	~WavPlayer(){
		sound.stop();
	}
};

#endif /* WavPlayer_hpp */
