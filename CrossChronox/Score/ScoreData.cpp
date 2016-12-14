//
//  ScoreData.cpp
//  CrossChronox
//
//  Created by HY_RORRE on 2016/10/10.
//  Copyright © 2016年 hyrorre. All rights reserved.
//

#include "ScoreData.hpp"


pulse_t ScoreData::MsToPulse(ms_type ms) const{
	double min = MsToMin(ms);
	ms_type total_pulse = 0;
	BpmEvent* last = bpm_events.cbegin()->get();
	BpmEvent* event = last + 1;
	BpmEvent* end = bpm_events.cend()->get();
	for(; event != end; ++event, ++last){
		double duration_min = (event->y - last->y) / (info.resolution * last->bpm);
		if(duration_min < min){
			min -= duration_min;
			total_pulse += last->bpm * duration_min * info.resolution;
		}
		else{
			break;
		}
	}
	total_pulse += last->bpm * min * info.resolution;
	return total_pulse;
}
