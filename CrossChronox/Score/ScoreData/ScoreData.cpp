//
//  ScoreData.cpp
//  CrossChronox
//
//  Created by HY_RORRE on 2016/10/10.
//  Copyright © 2016年 hyrorre. All rights reserved.
//

#include "ScoreData.hpp"


pulse_t ScoreData::MsToPulse(ms_type ms) const{
	min_type min = MsToMin(ms);
	pulse_t total_pulse = 0;
	BpmEvent* last = nullptr;
	for(auto& event : bpm_events){
		if(last){
			min_type duration_min = (event->pulse - last->pulse) / (info.resolution * last->bpm);
			if(duration_min < min){
				min -= duration_min;
				total_pulse += last->bpm * duration_min * info.resolution;
			}
			else{
				break;
			}
		}
		last = event.get();
	}
	total_pulse += last->bpm * min * info.resolution;
	return total_pulse;
}
