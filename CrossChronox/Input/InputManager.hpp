//
//  InputManager.hpp
//  CrossChronox
//
//  Created by HY_RORRE on 10/4/16.
//  Copyright © 2016 hyrorre. All rights reserved.
//

#ifndef InputManager_hpp
#define InputManager_hpp

#include "pch.hpp"



namespace InputManager{
	using frame_t = long;
	
	//pressing frames(+), or releasing frames(-)
	//0 when error occurs
	struct KeyState{
		frame_t now;
		frame_t last;
		
		KeyState(): now(-1000), last(-1000){}
		//KeyState(frame_t now, frame_t last): now(now), last(last){}
	};
	
	void LoadConfig(const std::string& json_path);
	
	void SetMode(const std::string& mode);
	
	//Call this func each frame
	void Update();
	
	KeyState GetKeyState(const std::string& key_str);
	KeyState GetKeyFuncState(const std::string& key_func_str);
};

#endif /* InputManager_hpp */
