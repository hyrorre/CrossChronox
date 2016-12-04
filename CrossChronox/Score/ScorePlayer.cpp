//
//  ScorePlayer.cpp
//  CrossChronox
//
//  Created by HY_RORRE on 2016/11/04.
//  Copyright © 2016年 hyrorre. All rights reserved.
//

#include "ScorePlayer.hpp"
#include "TimeManager.hpp"

ScorePlayer::ScorePlayer(const ScoreData& score_ref): score(&score_ref){
}

void ScorePlayer::SetScore(const ScoreData& score){
	this->score = &score;
}

bool ScorePlayer::Start(){
	start_ms = now_ms;
	return true;
}

bool ScorePlayer::Update(){
	ms_type play_ms = now_ms - start_ms;
	ms_type last_play_ms = play_ms - delta_ms;
	pulse_t now_pulse = score->MsToPulse(play_ms);
	pulse_t last_pulse = score->MsToPulse(last_play_ms);
	
//	Note tmp_note;
//	tmp_note.y = last_pulse;
//	auto begin = score->all_note.begin();
//	auto end = score->all_note.end();
//	auto pred = [](const Note* a, const Note* b){ return a->y < b->y; };
//	auto note = std::upper_bound(begin, end, tmp_note, pred);
//	
//	for(; note != end; ++note){
//		if(now_pulse < note->y) break;
//		
//	}
	
//	// last_frameで処理したNoteより後のNoteだけを処理
	Note tmp_note;
	tmp_note.y = last_pulse;
//	Note* end = *score->all_note.end();
//	auto pred = [](const Note* a, const Note* b){ return a->y < b->y; };
//	Note* note = *boost::upper_bound(score->all_note, tmp_note, pred);
//	for(; note != end; ++note){
//		if(now_pulse < note->y) break;
//		
//	}
	
	for(auto& channel : score->sound_channels){
		Note* note = &*boost::upper_bound(channel.notes, tmp_note);
		Note* end = &*channel.notes.end();
		for(; note != end; ++note){
			if(now_pulse < note->y) break;
			Note* next = channel.GetNextNote(note);
			ms_type len = 0;
			if(next){
				len = next->ms - note->ms;
			}
			
		}
		
	}
}

