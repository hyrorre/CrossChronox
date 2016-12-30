//
//  SelectMusic.cpp
//  CrossChronox
//
//  Created by HY_RORRE on 2016/12/22.
//  Copyright © 2016年 hyrorre. All rights reserved.
//

#include "SelectMusic.hpp"
#include "DefaultFont.hpp"
#include "InputManager.hpp"
#include "Application.hpp"
#include "PlayScore.hpp"

SelectMusic scene_select_music;

SelectMusic* scene_select_music_ptr = &scene_select_music;

Scene* SelectMusic::Update(){
	if(InputManager::GetKeyFuncState("UpMusic").now == 1){
		--cursor;
	}
	if(InputManager::GetKeyFuncState("DownMusic").now == 1){
		++cursor;
	}
	if(InputManager::GetKeyFuncState("CloseFolder").now == 1){
		if(now_directory->GetParent()){
			now_directory = now_directory->GetParent();
		}
	}
	if(InputManager::GetKeyFuncState("DecideMusic").now == 1){
		ScoreInfoBase* tmp_info = now_directory->At(cursor);
		if(typeid(*tmp_info) == typeid(ScoreDirectoryInfo)){
			ScoreDirectoryInfo* tmp_directory = static_cast<ScoreDirectoryInfo*>(tmp_info);
			if(!tmp_directory->Empty()){
				now_directory = tmp_directory;
			}
		}
		else{
			Application::SetScoreFilePath(tmp_info->path);
			return scene_play_score_ptr;
		}
	}
	return scene_select_music_ptr;
}

sf::Text text_songlist;

void SelectMusic::Init(){
	if(!inited){
		inited = true;
		std::string path = (Path::appdata / "Database/Song.xml").string();
		std::ifstream ifs(path);
		if(!ifs){ //if score info cache does not exist
			ScoreDirectoryLoader().Load(Path::appdata / "Songs", &root);
			std::ofstream ofs(path);
			boost::archive::xml_oarchive oarchive(ofs);
			oarchive.register_type<ScoreInfo>();
			oarchive.register_type<ScoreDirectoryInfo>();
			oarchive << boost::serialization::make_nvp("root", root);
		}
		else{ //cache exists
			boost::archive::xml_iarchive iarchive(ifs);
			iarchive.register_type<ScoreInfo>();
			iarchive.register_type<ScoreDirectoryInfo>();
			iarchive >> boost::serialization::make_nvp("root", root);
		}
		
		text_songlist.setFont(font_default);
		text_songlist.setPosition(320, 50);
	}
}

std::string str_songlist;

void SelectMusic::Draw(sf::RenderTarget& render_target) const{
	str_songlist.clear();
	for(int i = -2; i < 4; ++i){
		if(i == 0) str_songlist += "->";
		str_songlist += now_directory->At(i + cursor)->GetTitleSubtitle();
		str_songlist.push_back('\n');
		str_songlist.push_back('\n');
	}
	text_songlist.setString(str_songlist);
	render_target.draw(text_songlist);
}
