//
//  ScoreDirectoryInfo.cpp
//  CrossChronox
//
//  Created by HY_RORRE on 2016/12/22.
//  Copyright © 2016年 hyrorre. All rights reserved.
//

#include "ScoreDirectoryInfo.hpp"
#include "ScoreDirectoryLoader.hpp"
#include "Path.hpp"

std::string score_info_cache_path;

void SetScoreInfoCachePath(){
	// static bool inited = false;
	// if(!inited){
	// 	inited = true;
	// 	fs::path database = GetAppdataPath() / "Database";
	// 	if(!fs::exists(database)) fs::create_directories(database);
	// 	score_info_cache_path = (database / "Song.xml").string();
	// }
}

void ScoreDirectoryInfo::LoadScoreDirectory(){
	children.clear();
	ScoreDirectoryLoader().Load(GetAppdataPath() / "Songs", this);
}

void ScoreDirectoryInfo::SaveScoreDirectoryCache() const{
	// SetScoreInfoCachePath();
	// std::ofstream ofs(score_info_cache_path);
	// ofs.imbue(std::locale(""));
	//boost::archive::xml_oarchive oarchive(ofs);
	//oarchive.register_type<ScoreInfo>();
	//oarchive.register_type<ScoreDirectoryInfo>();
	//oarchive << boost::serialization::make_nvp("root", *this);
}

bool ScoreDirectoryInfo::TryLoadScoreDirectoryCache(){
	SetScoreInfoCachePath();
	std::ifstream ifs(score_info_cache_path);
	ifs.imbue(std::locale(""));
	if(!ifs){ //if score info cache does not exist
		return false;
	}
	else try{
		children.clear();
		//boost::archive::xml_iarchive iarchive(ifs);
		//iarchive.register_type<ScoreInfo>();
		//iarchive.register_type<ScoreDirectoryInfo>();
		
		//iarchive >> boost::serialization::make_nvp("root", *this);
		return true;
	}
	catch(std::exception& e){
		Clear();
		return false;
	}
}
