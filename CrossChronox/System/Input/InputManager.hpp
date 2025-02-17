#ifndef InputManager_hpp
#define InputManager_hpp

#include "pch.hpp"
#include "System/TimeManager.hpp"


namespace InputManager{
	using frame_t = long;
	
	struct KeyState{
		//pressing frames(+), or releasing frames(-)
		//0 when error occurs
		frame_t now = -1024;
		
		ms_type last_switched_ms = 0;
	};
	
	//Call these funcs before using
	void LoadConfig(const std::string& json_path);
	void SetMode(const std::string& mode);
	
	//Call this func each frame
	void Update();
	
	KeyState GetKeyState(const std::string& key_str);
	KeyState GetKeyFuncState(const std::string& key_func_str);
};

#endif /* InputManager_hpp */
