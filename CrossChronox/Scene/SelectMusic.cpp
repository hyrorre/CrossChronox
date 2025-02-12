//
//  SelectMusic.cpp
//  CrossChronox
//
//  Created by HY_RORRE on 2016/12/22.
//  Copyright © 2016年 hyrorre. All rights reserved.
//

#include "SelectMusic.hpp"
#include "System/DefaultFont.hpp"
#include "System/Input/InputManager.hpp"
#include "Application.hpp"
#include "PlayScore.hpp"

SelectMusic scene_select_music;

SelectMusic* scene_select_music_ptr = &scene_select_music;

sf::Text text_songlist;
sf::Text text_info;

Scene* SelectMusic::Update(){
	if(InputManager::GetKeyFuncState("Option").now > 0){
		// TODO: 複数プレイヤーおよびDPに対応
		int player = 0;
		PlayOption& option = players[player].GetVariableAccount().info.GetVariablePlayOption();
		Side side = LEFT;
		if(InputManager::GetKeyFuncState("IncrPlacement").now == 1){
			option.AddPlacement(side, 1);
		}
		if(InputManager::GetKeyFuncState("DecrPlacement").now == 1){
			option.AddPlacement(side, -1);
		}
		if(InputManager::GetKeyFuncState("IncrGaugeType").now == 1){
			option.AddGaugeType(1);
		}
		if(InputManager::GetKeyFuncState("DecrGaugeType").now == 1){
			option.AddGaugeType(-1);
		}
		if(InputManager::GetKeyFuncState("IncrAssistType").now == 1){
			option.AddAssistType(1);
		}
		if(InputManager::GetKeyFuncState("IncrFlip").now == 1){
			option.SetFlip(!option.GetFlip());
		}
		if(InputManager::GetKeyFuncState("IncrDisplayArea").now == 1){
			option.AddDisplayArea(1);
		}
	}
	else{
		if(InputManager::GetKeyFuncState("UpMusic").now == 1){
			now_directory->UpMusic();
			text_info.setString(now_directory->At(0)->GetInfoStr());
		}
		if(InputManager::GetKeyFuncState("DownMusic").now == 1){
			now_directory->DownMusic();
			text_info.setString(now_directory->At(0)->GetInfoStr());
		}
		if(InputManager::GetKeyFuncState("CloseFolder").now == 1){
			if(now_directory->GetParent()){
				now_directory = now_directory->GetParent();
				text_info.setString("");
			}
		}
		if(InputManager::GetKeyFuncState("DecideMusic").now == 1){
			ScoreInfoBase* tmp_info = now_directory->At(0);
			if(typeid(*tmp_info) == typeid(ScoreDirectoryInfo)){
				ScoreDirectoryInfo* tmp_directory = static_cast<ScoreDirectoryInfo*>(tmp_info);
				if(!tmp_directory->Empty()){
					now_directory = tmp_directory;
					text_info.setString(now_directory->At(0)->GetInfoStr());
				}
			}
			else{
				Application::SetScoreFilePath(tmp_info->path);
				return scene_play_score_ptr;
			}
		}
		if(InputManager::GetKeyFuncState("ReloadFolder").now == 1){
			root.LoadScoreDirectory();
			root.SaveScoreDirectoryCache();
		}
	}
	return scene_select_music_ptr;
}

void SelectMusic::Init(){
	if(!inited){
		inited = true;
		if(!root.TryLoadScoreDirectoryCache()){
			root.LoadScoreDirectory();
			root.SaveScoreDirectoryCache();
		}
		
		text_songlist.setFont(font_default);
		text_songlist.setPosition(1000, 50);
		text_songlist.setScale(1.5, 1.5);
		
		text_info.setFont(font_default);
		text_info.setCharacterSize(40);
	}
}

std::wstring str_songlist;

void SelectMusic::Draw(sf::RenderTarget& render_target) const{
	if(InputManager::GetKeyFuncState("Option").now > 0){
		
	}
	else{
		str_songlist.clear();
		for(int i = -3; i < 5; ++i){
			if(i == 0) str_songlist += L'▶';
			str_songlist += now_directory->At(i)->GetTitleSubtitle();
			str_songlist += L"\n\n";
		}
		text_songlist.setString(str_songlist);
		render_target.draw(text_songlist);
		render_target.draw(text_info);
	}
}
