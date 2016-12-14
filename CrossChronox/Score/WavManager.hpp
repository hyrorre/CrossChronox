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

class WavManager{
    using container_t = std::forward_list<std::unique_ptr<WavPlayer>>;
	
	container_t player;
};

#endif /* WavManager_hpp */
