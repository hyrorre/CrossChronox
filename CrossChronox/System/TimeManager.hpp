//
//  TimeManager.hpp
//  CrossChronox
//
//  Created by HY_RORRE on 2016/11/04.
//  Copyright © 2016年 hyrorre. All rights reserved.
//

#ifndef TimeManager_hpp
#define TimeManager_hpp

#include "pch.hpp"

namespace TimeManager{
	extern const sf::Uint32& lastframe_ms;
	extern const sf::Uint32& now_ms;
	extern const sf::Uint32& delta_ms;
	
	void Update();
};

using namespace TimeManager;

#endif /* TimeManager_hpp */
