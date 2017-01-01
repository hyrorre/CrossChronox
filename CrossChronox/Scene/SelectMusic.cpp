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
		now_directory->UpMusic();
	}
	if(InputManager::GetKeyFuncState("DownMusic").now == 1){
		now_directory->DownMusic();
	}
	if(InputManager::GetKeyFuncState("CloseFolder").now == 1){
		if(now_directory->GetParent()){
			now_directory = now_directory->GetParent();
		}
	}
	if(InputManager::GetKeyFuncState("DecideMusic").now == 1){
		ScoreInfoBase* tmp_info = now_directory->At(0);
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
		fs::path database = Path::appdata / "Database";
		if(!fs::exists(database)) fs::create_directories(database);
		std::string path = (database / "Song.xml").string();
		bool load_score_directory_flag = false;
		{
			std::ifstream ifs(path);
			ifs.imbue(std::locale(""));
			if(!ifs){ //if score info cache does not exist
				load_score_directory_flag = true;
			}
			else{ //cache exists
				try{
					boost::archive::xml_iarchive iarchive(ifs);
					iarchive.register_type<ScoreInfo>();
					iarchive.register_type<ScoreDirectoryInfo>();
					iarchive >> boost::serialization::make_nvp("root", root);
				}
				catch(std::exception& e){
					root.Clear();
					load_score_directory_flag = true;
				}
			}
		}
		if(load_score_directory_flag){
			ScoreDirectoryLoader().Load(Path::appdata / "Songs", &root);
			std::ofstream ofs(path);
			ofs.imbue(std::locale(""));
			boost::archive::xml_oarchive oarchive(ofs);
			oarchive.register_type<ScoreInfo>();
			oarchive.register_type<ScoreDirectoryInfo>();
			oarchive << boost::serialization::make_nvp("root", root);
		}
		
		text_songlist.setFont(font_default);
		text_songlist.setPosition(320, 50);
	}
}

std::wstring str_songlist;

void SelectMusic::Draw(sf::RenderTarget& render_target) const{
	str_songlist.clear();
	for(int i = -2; i < 4; ++i){
		if(i == 0) str_songlist += L"->";
		str_songlist += now_directory->At(i)->GetTitleSubtitle();
		str_songlist += L"\n\n";
	}
	text_songlist.setString(str_songlist);
	render_target.draw(text_songlist);
}
