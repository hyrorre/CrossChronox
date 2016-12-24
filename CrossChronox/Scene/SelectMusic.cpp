//
//  SelectMusic.cpp
//  CrossChronox
//
//  Created by HY_RORRE on 2016/12/22.
//  Copyright © 2016年 hyrorre. All rights reserved.
//

#include "SelectMusic.hpp"
#include "DefaultFont.hpp"

SelectMusic scene_select_music;

SelectMusic* scene_select_music_ptr = &scene_select_music;

Scene* SelectMusic::Update(){
	return scene_select_music_ptr;
}

sf::Text text_songlist;

void SelectMusic::Init(){
	if(!inited){
		inited = true;
		ScoreDirectoryLoader().Load(Path::appdata / "Songs", &root);
		
		text_songlist.setFont(font_default);
		text_songlist.setPosition(320, 50);
	}
}

std::string str_songlist;

void SelectMusic::Draw(sf::RenderTarget& render_target) const{
	str_songlist.clear();
	for(int i = -2; i < 4; ++i){
		if(i == 0) str_songlist += "->";
		str_songlist += now_directory->At(i)->GetTitleSubtitle();
		str_songlist.push_back('\n');
		str_songlist.push_back('\n');
	}
	text_songlist.setString(str_songlist);
	render_target.draw(text_songlist);
}
