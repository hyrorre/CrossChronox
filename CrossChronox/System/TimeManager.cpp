//
//  TimeManager.cpp
//  CrossChronox
//
//  Created by HY_RORRE on 2016/11/04.
//  Copyright © 2016年 hyrorre. All rights reserved.
//

#include "TimeManager.hpp"



namespace TimeManager{
	sf::Uint32 lastframe_ms_ = 0;
	sf::Uint32 now_ms_ = 0;
	sf::Uint32 delta_ms_ = 0;
	
	const sf::Uint32& lastframe_ms = lastframe_ms_;
	const sf::Uint32& now_ms = now_ms_;
	const sf::Uint32& delta_ms = delta_ms_;
	
	void Update(){
		static const sf::Clock g_clock;
		lastframe_ms_ = now_ms_;
		now_ms_ = g_clock.getElapsedTime().asMilliseconds();
		delta_ms_ = now_ms_ - lastframe_ms_;
	}
}
