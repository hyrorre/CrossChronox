//
//  PlayScore.cpp
//  CrossChronox
//
//  Created by HY_RORRE on 2016/10/10.
//  Copyright © 2016年 hyrorre. All rights reserved.
//

#include "PlayScore.hpp"

PlayScore scene_play_score;

PlayScore* scene_play_score_ptr = &scene_play_score;

void PlayScore::Deinit(){
	//これが無いと恐らくリソースの解放順の問題で強制終了する
	for(auto& player : players){
		player.Clear();
	}
}

Scene* PlayScore::Update(){
	int continue_flag = 0;
	for(auto& player : players){
		continue_flag += player.Update();
	}
	if(continue_flag) return scene_play_score_ptr;
	else return nullptr;
}

float scr_w = 60;
float white_w = 30;
float black_w = 20;

float note_h = 6;

float space = 2;

float scr_x = 34;

float judgeline_w = scr_w + white_w * 4 + black_w * 3 + space * 7;

float judgeline_y = 640 - 165;

sf::Texture white_shape;
sf::Sprite white_sprite;

sf::Texture black_shape;
sf::Sprite black_sprite;

sf::Texture scr_shape;
sf::Sprite scr_sprite;

sf::Texture judgeline_shape;
sf::Sprite judgeline_sprite;

void PlayScore::Init(){
	sf::Image tmp_image;
	tmp_image.create(white_w, note_h, sf::Color::White);
	white_shape.loadFromImage(tmp_image);
	white_sprite.setTexture(white_shape);
	tmp_image.create(black_w, note_h, sf::Color::Cyan);
	black_shape.loadFromImage(tmp_image);
	black_sprite.setTexture(black_shape);
	tmp_image.create(scr_w, note_h, sf::Color::Magenta);
	scr_shape.loadFromImage(tmp_image);
	scr_sprite.setTexture(scr_shape);
	tmp_image.create(judgeline_w, note_h, sf::Color::Red);
	judgeline_shape.loadFromImage(tmp_image);
	judgeline_sprite.setTexture(judgeline_shape);
	judgeline_sprite.setPosition(scr_x, judgeline_y);
	
	for(auto& player : players){
		player.Init();
	}
	ScorePlayer::Start();
}

float GetNoteX(Note::lane_t lane){
	switch(lane){
		case 8:
			return scr_x;
		case 1:
		case 2:
		case 3:
		case 4:
		case 5:
		case 6:
		case 7:
			return scr_x + scr_w + int(lane / 2) * white_w + int((lane - 1) / 2) * black_w + lane * space;
		default:
			//qDebug("lane:%d", lane);
			return 0;
	}
}

sf::Sprite* GetSpritePtr(Note::lane_t lane){
	switch(lane){
		case 8:
			return &scr_sprite;
		case 1:
		case 3:
		case 5:
		case 7:
			return &white_sprite;
		case 2:
		case 4:
		case 6:
			return &black_sprite;
		default:
			//qDebug("lane:%d", lane);
			return nullptr;
	}
}

float global_scroll = .7f * 240;

void PlayScore::Draw(sf::RenderTarget& render_target) const{
	render_target.draw(judgeline_sprite);
	for(auto& player : players){
		ms_type play_ms = player.GetPlayMs();
		//ms_type last_play_ms = play_ms - delta_ms;
		pulse_t now_pulse = player.GetScore().MsToPulse(play_ms);
		//pulse_t last_pulse = player.GetScore().MsToPulse(last_play_ms);
		
		const ScoreData& score = player.GetScore();
		for(const auto& note : score.notes){
			if(note->y < now_pulse) continue;
			sf::Sprite* sprite = GetSpritePtr(note->x);
			if(sprite){
				float note_y = judgeline_y - static_cast<int>((note->y - now_pulse) * global_scroll / score.info.resolution);
				if(note_y + note_h < 0) break;
				sprite->setPosition(GetNoteX(note->x), note_y);
				render_target.draw(*sprite);
			}
		}
	}
}


