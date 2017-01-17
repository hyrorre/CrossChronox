//
//  InputManager.cpp
//  CrossChronox
//
//  Created by HY_RORRE on 10/4/16.
//  Copyright © 2016 hyrorre. All rights reserved.
//

#include "InputManager.hpp"
#include "Exception.hpp"

namespace InputManager{
	
	//keyid
	//0~999 keyid of keyboard(sf::Keyboard)
	//1000~1999 (下二桁)keyid of joystick 1
	//2000~2999 (下二桁)keyid of joystick 2
	using keyid_t = int;
	
	bool IsKeyPressed(keyid_t keyid){
		if(keyid < 1000){ //keyboard
			return sf::Keyboard::isKeyPressed(static_cast<sf::Keyboard::Key>(keyid));
		}
		else{ //joystick
			unsigned joyid = (keyid / 1000) - 1;
			return sf::Joystick::isButtonPressed(joyid, keyid % 1000);
		}
	};
	
	using keymap_t = std::unordered_map<std::string, std::vector<keyid_t>>;
	
	using keyfuncmap_t = std::unordered_map<std::string, std::vector<std::string>>;
	
	std::unordered_map<keyid_t, KeyState> key_states;
	
	struct Mode{
		keymap_t keymap;
		keyfuncmap_t keyfuncmap;
	};
	
	using modemap_t = std::unordered_map<std::string, Mode>;
	modemap_t modemap;
	
	Mode* now_mode = nullptr;
	
	void LoadConfig(const std::string& json_path){
		std::ifstream ifs(json_path);
		if(!ifs){
			throw OpenError(std::string("\"") + json_path + "\" could be not opened.");
		}
		picojson::value root_value;
		picojson::parse(root_value, ifs);
		
		auto& root = root_value.get<picojson::object>();
		
		for(auto& mode_pair : root){
			auto& mode = modemap[mode_pair.first];
			auto& keymap = mode_pair.second.get<picojson::object>()["Key"].get<picojson::object>();
			for(auto& key_pair : keymap){
				auto& keyids = mode.keymap[key_pair.first];
				for(auto keyid : key_pair.second.get<picojson::array>()){
					keyid_t tmp = static_cast<keyid_t>(keyid.get<int64_t>());
					keyids.emplace_back(tmp);
					key_states[tmp];
				}
				auto& keyfuncmap = mode_pair.second.get<picojson::object>()["KeyFunc"].get<picojson::object>();
				for(auto& func_pair : keyfuncmap){
					auto& keynames = mode.keyfuncmap[func_pair.first];
					for(auto& keyname : func_pair.second.get<picojson::array>()){
						keynames.emplace_back(keyname.get<std::string>());
					}
				}
			}
		}
	}
	
	void SetMode(const std::string& mode){
		try{
			now_mode = &modemap.at(mode);
		}
		catch(std::out_of_range& e){
			throw std::runtime_error(std::string("The mode named " + mode + " was not found or configured.\n") + e.what());
		}
	}
	
	void Update(){
		for(auto& key_state : key_states){
			const keyid_t& keyid = key_state.first;
			auto& state = key_state.second;
			
			state.last = state.now;
			if(0 < state.now){ //last frame, key was pressed
				if(IsKeyPressed(keyid)) ++state.now;
				else state.now = -1;
			}
			else{ //last frame, key was released
				if(IsKeyPressed(keyid)) state.now = 1;
				else --state.now;
			}
		}
	}
	
	KeyState GetKeyState(const std::string& key_str){
		assert(now_mode);
		KeyState ret;
		for(auto keyid : now_mode->keymap[key_str]){
			auto& state = key_states[keyid];
			ret.now = std::max(ret.now, state.now);
			ret.last = std::max(ret.last, state.last);
		}
		return ret;
	}
	
	KeyState GetKeyFuncState(const std::string& key_func_str){
		assert(now_mode);
		KeyState ret;
		for(auto& key_str : now_mode->keyfuncmap[key_func_str]){
			KeyState state = GetKeyState(key_str);
			ret.now = std::max(ret.now, state.now);
			ret.last = std::max(ret.last, state.last);
		}
		return ret;
	}
}
