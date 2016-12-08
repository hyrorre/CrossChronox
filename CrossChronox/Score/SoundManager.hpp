//
//  SoundManager.hpp
//  CrossChronox
//
//  Created by HY_RORRE on 2016/11/28.
//  Copyright © 2016年 hyrorre. All rights reserved.
//

#ifndef SoundManager_hpp
#define SoundManager_hpp

#include "pch.hpp"
#include "SoundPlayer.hpp"

class SoundManager{
	using container_t = boost::ptr_list<SoundPlayer>;
	
	container_t player;
};

#endif /* SoundManager_hpp */
