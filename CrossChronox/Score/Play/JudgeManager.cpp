#include "JudgeManager.hpp"

namespace JudgeManager{
	//enum{ MAX_JUDGE_TABLE_VALUE = 6 };
	
	
	// BMSの各判定の範囲(+-ms)。PGREAT, GREAT, GOOD, BAD, MISS空POORの順
	const int bms_judge_table[] = { 20, 60, 160, 250, 1000 };
	
	// BMSのLN終端の各判定の範囲(+-ms)。PGREAT, GREAT, GOOD, BAD, MISS空POORの順
	const int bms_lnend_judge_table[] = { 100, 150, 200, 250, 1000 };
	
	// BMSのスクラッチの各判定の範囲(+-ms)。PGREAT, GREAT, GOOD, BAD, MISS空POORの順
	const int scr_judge_table[] = { 30, 75, 200, 300, 1000 };
	
	// BMSのBSS終端の各判定の範囲(+-ms)。PGREAT, GREAT, GOOD, BAD, MISS空POORの順
	const int bssend_judge_table[] = { 100, 150, 250, 300, 1000 };
	
	// PMSの各判定の範囲(+-ms)。PGREAT, GREAT, GOOD, BAD, MISS空POORの順
	const int pms_judge_table[] = { 25, 75, 175, 200, 1000 };
	
	// PMSのLN終端の各判定の範囲(+-ms)。PGREAT, GREAT, GOOD, BAD, MISS空POORの順
	const int pms_lnend_judge_table[] = { 100, 150, 175, 200, 1000 };
	
	const int* GetJudgeTable(lane_t lane, bool lnend_flag, Mode mode){
		if(mode <= BEAT_14K){ // if bms
			if(lane == 8){ // if scratch
				return (lnend_flag ? bssend_judge_table : scr_judge_table);
			}
			// if normal note
			else return (lnend_flag ? bms_lnend_judge_table : bms_judge_table);
		}
		// if pms
		else return (lnend_flag ? pms_lnend_judge_table : pms_judge_table);
	}
	
//	int judge_algorithm = JUDGE_ALGORITHM_LR2;
//	
//	void SetJudgeAlgorithm(int value){
//		judge_algorithm = value;
//	}
	
	Note* UpdateLane(const std::vector<Note*>& lane_timeline, InputManager::KeyState key_state, ms_type play_ms, lane_t lane, Mode mode, LnType ln_type, std::vector<NoteJudge>* out){
		Note* pending_note = nullptr;
		Note* last_note = nullptr;
		out->clear(); // initialize out
		if(!lane_timeline.empty()){
			const int* judge_table = GetJudgeTable(lane_timeline.front()->lane, false, mode);
			for(Note* note : lane_timeline){
				// if note is already judged, judge next note
				if(note->judge == JUDGE_YET){
					// ms_type is not appropriate becase it is unsigned
					auto diff_ms = static_cast<int>(note->ms - play_ms);
					
					// handle overlooked notes (見逃しPOORの判定)
					if(judge_table[BAD] < -diff_ms){
						out->emplace_back(note, POOR, true);
					}
					
					else{
						// laneに対応するキーが押された瞬間であるとき
						if(key_state.now == 1){
							int abs_diff_ms = abs(diff_ms);
							
							// GOOD範囲内にノーツがあったら判定処理
							for(Judge judge = PGREAT; judge <= GOOD; judge = Judge(judge + 1)){
								if(abs_diff_ms <= judge_table[judge]){
									// LNのときjudgeをLN_PUSHINGにする
									out->emplace_back(note, static_cast<Judge>(judge - (note->len ? 10 : 0)), false);
									return note;
								}
							}
							
							// BAD範囲内のノーツは保留 (以降のノーツがGOOD範囲内だったらそちらを先に処理)
							if(abs_diff_ms <= judge_table[BAD]){
								if(pending_note == nullptr) pending_note = note;
							}
							// 空POOR判定
							else if(abs_diff_ms <= judge_table[POOR]){
								out->emplace_back(note, POOR, false);
								return note;
							}
							// 保留中だったBAD範囲内のノーツを判定してからreturn
							else if(pending_note){
								out->emplace_back(pending_note, BAD, true);
								return pending_note;
							}
							// 保留中のBAD範囲内ノーツが無かった場合、音だけ鳴らす
							// last_noteが無い場合(1ノーツ目が振ってきていない場合)、nullptrが返る
							else return last_note;
						}
						
						// キーが押された瞬間でないとき
						else return nullptr;
					}
				}
				// LNが押されている時
				else if(note->judge < JUDGE_YET){
					// キーが押されたままのとき
					bool ln_finish_pushing_flag = false;
					if(0 < key_state.now){
						// lnendを過ぎたら
						if(note->lnend_ms < play_ms) ln_finish_pushing_flag = true;
					}
					// キーが離された時
					else{
						// lnendに近いところで離したらBADにはならない
						if(note->lnend_ms < play_ms + GetJudgeTable(note->lane, true, mode)[GOOD]) ln_finish_pushing_flag = true;
						// 離すのが早すぎるとBAD
						else{
							out->emplace_back(note, BAD, true, true);
						}
						
					}
					if(ln_finish_pushing_flag){
						auto judge = static_cast<Judge>(note->judge + 10);
						out->emplace_back(note, judge, false);
					}
				}
				
				// update last_note
				last_note = note;
			} // end of for-loop
		}
		return nullptr;
	}
}
