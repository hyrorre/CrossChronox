﻿//
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
	using ms_type = std::uint32_t;
	using sec_type = double;
	using min_type = double;
	
	extern const ms_type& lastframe_ms;
	extern const ms_type& now_ms;
	extern const ms_type& delta_ms;
	
	void Update();
	
	inline min_type MsToMin(ms_type ms){
		return ms / (1000.0 * 60.0);
	}
	inline ms_type MinToMs(min_type min){
		return min * 1000.0 * 60.0;
	}
	
	ms_type GetRealtime();
};

using namespace TimeManager;

#endif /* TimeManager_hpp */
