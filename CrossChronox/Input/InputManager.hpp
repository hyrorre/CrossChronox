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

class InputManager{
public:
	static bool LoadConfig(const boost::string_ref& json_path);
	
	//Call this func each frame
	static void Update();
	
	//Get pressing frames(+), or releasing frames(-)
	static int GetState(const boost::string_ref& key_func);
};

#endif /* InputManager_hpp */
