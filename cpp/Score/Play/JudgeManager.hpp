#pragma once

#include "pch.hpp"
#include "Score/ScoreData/ScoreData.hpp"
#include "System/InputManager.hpp"

// enum{
//	JUDGE_ALGORITHM_LR2,
//	JUDGE_ALGORITHM_IIDX,
//	JUDGE_ALGORITHM_LOWEST_NOTE,
//
//	MAX_JUDGE_ALGORITHM
// };

namespace JudgeManager {
    struct NoteJudge {
        Note* note = nullptr;
        Judge judge = JUDGE_YET;
        bool cb_flag = false;
        bool stop_wav_flag = false;

        NoteJudge() {}
        NoteJudge(Note* note, Judge judge, bool cb_flag) : note(note), judge(judge), cb_flag(cb_flag) {}
        NoteJudge(Note* note, Judge judge, bool cb_flag, bool stop_wav_flag) : note(note), judge(judge), cb_flag(cb_flag), stop_wav_flag(stop_wav_flag) {}
    };

    // void SetJudgeAlgorithm(int value);

    // this func does not modify lane_timeline directly
    // so modify it later with argument named "out"
    // return pointer to note which sound should be started to play
    Note* UpdateLane(const std::vector<Note*>& lane_timeline, InputManager::KeyState key_state, ms_type play_ms, lane_t lane, Mode mode, LnType ln_type, std::vector<NoteJudge>* out);
} // namespace JudgeManager
