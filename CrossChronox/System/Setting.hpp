//
//  Setting.hpp
//  CrossChronox
//
//  Created by HY_RORRE on 2017/04/15.
//  Copyright © 2017年 hyrorre. All rights reserved.
//

#ifndef Setting_hpp
#define Setting_hpp

#include "pch.hpp"

enum{
	WINDOWED,
	FULLSCREEN,
	BORDERLESS,
};

class Setting{
	int window_type = WINDOWED;
	bool save_resolution = true;
	int resolution_x = 800;
	int resolution_y = 600;
	bool save_window_size = true;
	int window_size_x = 800;
	int window_size_y = 600;
	bool save_window_pos = true;
	int window_pos_x = 200;
	int window_pos_y = 200;
	bool vsync = true;
	int max_fps = 0; // if max_fps is 0, fps is unlimited
	
	std::vector<std::string> song_paths;
	std::vector<std::string> table_urls;
	
public:
	int GetWindowType() const{
		return window_type;
	}
	int GetResolutionX() const{
		return resolution_x;
	}
	int GetResolutionY() const{
		return resolution_y;
	}
	sf::Vector2i GetResolution() const{
		return sf::Vector2i(GetResolutionX(), GetResolutionY());
	}
	int GetWindowSizeX() const{
		return window_size_x;
	}
	int GetWindowSizeY() const{
		return window_size_y;
	}
	sf::Vector2i GetWindowSize() const{
		return sf::Vector2i(GetWindowSizeX(), GetWindowSizeY());
	}
	int GetWindowsPosX() const{
		return window_pos_x;
	}
	int GetWindowPosY() const{
		return window_pos_y;
	}
	sf::Vector2i GetWindowPos() const{
		return sf::Vector2i(GetWindowsPosX(), GetWindowPosY());
	}
	void SetWindowPos(const sf::Vector2i& value){
		if(save_window_pos){
			window_pos_x = value.x;
			window_pos_y = value.y;
		}
	}
	bool GetVsync() const{
		return vsync;
	}
	int GetMaxFps() const{
		return max_fps;
	}
	
	bool TryLoadFile(const std::string& filepath){
		try{
			std::ifstream ifs(filepath);
			if(!ifs){
				return false;
			}
			boost::archive::xml_iarchive iarchive(ifs);
			iarchive >> boost::serialization::make_nvp("root", *this);
			return true;
		}
		catch(std::exception& e){
			return false;
		}
	}
	
	void SaveFile(const std::string& filepath){
		std::ofstream ofs(filepath);
		boost::archive::xml_oarchive oarchive(ofs);
		oarchive << boost::serialization::make_nvp("root", *this);
	}
	
private:
	friend class boost::serialization::access;
	template<class Archive>
	void serialize(Archive& ar, const unsigned int version){
		ar & BOOST_SERIALIZATION_NVP(window_type);
		ar & BOOST_SERIALIZATION_NVP(save_resolution);
		ar & BOOST_SERIALIZATION_NVP(resolution_x);
		ar & BOOST_SERIALIZATION_NVP(resolution_y);
		ar & BOOST_SERIALIZATION_NVP(save_window_pos);
		ar & BOOST_SERIALIZATION_NVP(window_pos_x);
		ar & BOOST_SERIALIZATION_NVP(window_pos_y);
		ar & BOOST_SERIALIZATION_NVP(save_window_size);
		ar & BOOST_SERIALIZATION_NVP(window_size_x);
		ar & BOOST_SERIALIZATION_NVP(window_size_y);
		ar & BOOST_SERIALIZATION_NVP(vsync);
		ar & BOOST_SERIALIZATION_NVP(max_fps);
		ar & BOOST_SERIALIZATION_NVP(song_paths);
		ar & BOOST_SERIALIZATION_NVP(table_urls);
	}
};

extern Setting setting;

BOOST_CLASS_VERSION(Setting, 1);


#endif /* Setting_hpp */
