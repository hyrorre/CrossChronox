//
//  PlayScore.cpp
//  CrossChronox
//
//  Created by HY_RORRE on 2016/10/10.
//  Copyright © 2016年 hyrorre. All rights reserved.
//

#include "PlayScore.hpp"
#include "Path.hpp"
#include "DefaultFont.hpp"

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

float judge_combo_x = scr_x + 40;

float judge_combo_y = judgeline_y - 130;

sf::Texture white_shape;
sf::Sprite white_sprite;

sf::Texture black_shape;
sf::Sprite black_sprite;

sf::Texture scr_shape;
sf::Sprite scr_sprite;

sf::Texture judgeline_shape;
sf::Sprite judgeline_sprite;

sf::Text text_fps;
sf::Text text_info_play;

sf::Clock clock_fps;
int fps_count = 0;

sf::Color color_judge_combo[] = {
	sf::Color::Cyan,   //PGREAT
	sf::Color::Yellow, //GREAT
	sf::Color::Yellow, //GOOD
	sf::Color::Red,    //BAD
	sf::Color::Red,    //POOR
};

std::string str_judge_combo[] = {
	"GREAT ", //PGREAT
	"GREAT ", //GREAT
	"GOOD ",  //GOOD
	"BAD ",   //BAD
	"POOR ",  //POOR
};

class JudgeCombo{
	const ScorePlayer* player;
	sf::Text text;
	Side side = LEFT;
	
	ms_type last_ms = 0;
	size_t last_combo = 0;
	
public:
	void Init(const ScorePlayer& player, Side side){
		this->side = side;
		this->player = &player;
		text.setFont(font_default);
		text.setPosition(judge_combo_x, judge_combo_y);
		text.setCharacterSize(40);
		text.setOutlineColor(sf::Color::Black);
		text.setOutlineThickness(1);
	}
	const sf::Text* GetText(){
		JudgeInfo info = player->GetResult().GetLastJudgeInfo(side);
		if(info.judge != JUDGE_YET){
			ms_type elapsed = player->GetPlayMs() - info.ms;
			if(elapsed < 2000 && (info.judge == Judge::PGREAT || (elapsed / 10) % 3 == 1)){
				text.setString(str_judge_combo[info.judge] + std::to_string(info.combo));
				text.setFillColor(color_judge_combo[info.judge]);
				return &text;
			}
		}
		return nullptr;
	}
};

std::array<JudgeCombo, MAX_SIDE> judge_combos;

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
	
	text_fps.setFont(font_default);
	text_fps.setFillColor(sf::Color::White);
	
	text_info_play.setFont(font_default);
	text_info_play.setCharacterSize(15);
	text_info_play.setPosition(400, 0);
	
	for(int i = 0; i < judge_combos.size(); ++i){
		judge_combos[i].Init(players[0], static_cast<Side>(i));
	}
	
	clock_fps.restart();
	
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

#define SS(x) ss << #x L": " << x << L'\n'

void PlayScore::Draw(sf::RenderTarget& render_target) const{
	std::wstringstream ss;
	
	render_target.draw(judgeline_sprite);
	for(auto& player : players){
		ms_type play_ms = player.GetPlayMs();
		ms_type last_play_ms = play_ms - delta_ms;
		pulse_t now_pulse = player.GetScore().MsToPulse(play_ms);
		pulse_t last_pulse = player.GetScore().MsToPulse(last_play_ms);
		
		SS(play_ms);
		SS(last_play_ms);
		SS(now_pulse);
		SS(last_pulse);
		
		const ScoreData& score = player.GetScore();
		ss << score.info.GetInfoStr();
		ss << player.GetResult().GetResultStr() << std::endl;
		for(const auto& note : score.notes){
			float lnstart_y = judgeline_y - static_cast<int>((note->y - now_pulse) * global_scroll / score.info.resolution);
			float note_y = judgeline_y - static_cast<int>((note->y + note->l - now_pulse) * global_scroll / score.info.resolution);
			if(judgeline_y + note_h < note_y) continue;
			if(lnstart_y + note_h < 0) break;
			sf::Sprite* sprite = GetSpritePtr(note->x);
			if(sprite){
				if(note->l == 0){ //if note is not LN
					sprite->setPosition(GetNoteX(note->x), note_y);
					render_target.draw(*sprite);
				}
				else{ //if note is LN
					sf::Sprite ln_sprite = *sprite;
					if(note->y < now_pulse){
						lnstart_y = judgeline_y;
					}
					float lnend_y = note_y;
					float note_x = GetNoteX(note->x);
					sprite->setPosition(note_x, lnstart_y);
					render_target.draw(*sprite);
					sprite->setPosition(note_x, lnend_y);
					render_target.draw(*sprite);
					ln_sprite.setPosition(note_x, lnend_y);
					ln_sprite.setScale(1, (lnstart_y - lnend_y) / note_h);
					render_target.draw(ln_sprite);
				}
			}
		}
	}
	++fps_count;
	if(sf::seconds(1) < clock_fps.getElapsedTime()){
		clock_fps.restart();
		std::stringstream ss;
		ss << fps_count << "fps";
		text_fps.setString(ss.str());
		fps_count = 0;
	}
	render_target.draw(text_fps);
	
	text_info_play.setString(ss.str());
	render_target.draw(text_info_play);
	
	const sf::Text* tmp = judge_combos[0].GetText();
	if(tmp){
		render_target.draw(*tmp);
	}
}


