//
//  ScorePlayer.cpp
//  CrossChronox
//
//  Created by HY_RORRE on 2016/11/04.
//  Copyright © 2016年 hyrorre. All rights reserved.
//

#include "ScorePlayer.hpp"
#include "System/TimeManager.hpp"
#include "Score/Load/BmsLoader.hpp"
#include "Application.hpp"
#include "JudgeManager.hpp"

//static variables
ms_type ScorePlayer::start_ms = 0;

void ScorePlayer::SetLaneTimelines(){
	for(auto& tl : lane_timelines) tl.clear();
	for(auto& note : score.notes){
		auto lane = note->lane;
		if(lane){
			assert(lane < MAX_LANE);
			lane_timelines[lane].emplace_back(note.get());
		}
	}
}

void ScorePlayer::Init(){
	LoadBms(Application::GetScoreFilePath().string(), &score);
	SetLaneTimelines();
	wav_manager.Init();
	result = Result();
}

void ScorePlayer::Start(){
	start_ms = TimeManager::GetRealtime();
}

std::unique_ptr<Note> tmp_note(new Note());

ScorePlayer::State ScorePlayer::Update(){
	ms_type play_ms = GetPlayMs();
	ms_type last_play_ms = play_ms - std::min(play_ms, delta_ms);
	pulse_t now_pulse = score.MsToPulse(play_ms);
	pulse_t last_pulse = score.MsToPulse(last_play_ms);
	
	wav_manager.Update();
	
	tmp_note->pulse = last_pulse;
	auto begin = score.notes.begin();
	auto end = score.notes.end();
	auto note_it = std::upper_bound(begin, end, tmp_note, ptr_less<Note>());
	for(; note_it != end; ++note_it){
		Note& note = *(*note_it);
		if(now_pulse < note.pulse) break;
		wav_manager.PlayWav(&note);
		if(note.num){
			Side side = static_cast<Side>(score.info.mode != Mode::POPN_9K && 9 <= note.lane);
			result.Push(side, play_ms, Judge::PGREAT, false);
		}
	}
	if(score.info.end_pulse < now_pulse && wav_manager.Empty()) return State::FINISH;
	else return State::CONTINUE;
}

const char* key_str[] = {
	"",       // lane 0
	"1pKey1", // lane 1
	"1pKey2", // lane 2
	"1pKey3", // lane 3
	"1pKey4", // lane 4
	"1pKey5", // lane 5
	"1pKey6", // lane 6
	"1pKey7", // lane 7
	
	"1pScr",  // lane 8
	
	"2pKey1", // lane 9
	"2pKey2", // lane 10
	"2pKey3", // lane 11
	"2pKey4", // lane 12
	"2pKey5", // lane 13
	"2pKey6", // lane 14
	"2pKey7", // lane 15
	
	"2pScr",  // lane 16
};

void ScorePlayer::Judge(){
	// lane 0 is BGM
	for(lane_t lane = 1; lane < MAX_LANE; ++lane){
		auto& lane_timeline = lane_timelines[lane];
		std::vector<JudgeManager::NoteJudge> notejudge;
		
		JudgeManager::UpdateLane(lane_timeline, InputManager::GetKeyFuncState(key_str[lane]), now_ms, lane, score.info.mode, score.info.ln_type, &notejudge);
		
	}
}


