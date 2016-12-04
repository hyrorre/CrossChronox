//
//  SoundPlayer.hpp
//  CrossChronox
//
//  Created by HY_RORRE on 2016/11/05.
//  Copyright © 2016年 hyrorre. All rights reserved.
//

#ifndef SoundPlayer_hpp
#define SoundPlayer_hpp

#include "pch.hpp"
#include "ScoreData.hpp"

class SoundPlayer : boost::noncopyable{
	SoundChannel& channel;
	//ms_type start_offset;
	ms_type duration;
	
	sf::Sound sound;
	ms_type start_ms;
	
public:
	SoundPlayer(SoundChannel& channel, ms_type start_offset, ms_type duration): channel(channel), duration(duration){
		sound.setBuffer(channel.buf);
		sound.setPlayingOffset(sf::milliseconds(start_offset));
    }
};

#endif /* SoundPlayer_hpp */
