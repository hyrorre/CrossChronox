//
//  ScorePlayer.cpp
//  CrossChronox
//
//  Created by HY_RORRE on 2016/11/04.
//  Copyright © 2016年 hyrorre. All rights reserved.
//

#include "ScorePlayer.hpp"
#include "TimeManager.hpp"
#include "BmsLoader.hpp"
#include "Application.hpp"

//static variables
ms_type ScorePlayer::start_ms = 0;

void ScorePlayer::Init(){
	LoadBms(Application::GetScoreFilePath().string(), &score);
	wav_manager.Init();
	result = Result();
}

void ScorePlayer::Start(){
	start_ms = now_ms;
}

ScorePlayer::State ScorePlayer::Update(){
	ms_type play_ms = now_ms - start_ms;
	ms_type last_play_ms = play_ms - delta_ms;
	pulse_t now_pulse = score.MsToPulse(play_ms);
	pulse_t last_pulse = score.MsToPulse(last_play_ms);
	
	wav_manager.Update();
	
	std::unique_ptr<Note> tmp_note(new Note());
	tmp_note->y = last_pulse;
	auto begin = score.notes.begin();
	auto end = score.notes.end();
	auto note_it = std::upper_bound(begin, end, tmp_note, ptr_less<Note>());
	for(; note_it != end; ++note_it){
		Note& note = *(*note_it);
		if(now_pulse < note.y) break;
		wav_manager.PlayWav(&note);
	}
	if(score.info.end_y < now_pulse) return State::FINISH;
	else return State::CONTINUE;
}

