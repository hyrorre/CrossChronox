#include "ScorePlayer.hpp"
#include "Application.hpp"
#include "JudgeManager.hpp"
#include "Score/Load/BmsLoader.hpp"
#include "System/TimeManager.hpp"

std::array<ScorePlayer, MAX_PLAYER> players;

// static variables
ms_type ScorePlayer::start_ms = 0;

void ScorePlayer::SetLaneTimelines() {
    for (auto& tl : lane_timelines)
        tl.clear();
    std::vector<lane_t> placements;
    if (score.info.mode == BEAT_7K) {
        placements.resize(7);
        if (players[0].GetVariableAccount().info.GetPlayOption().GetPlacement(LEFT) == RANDOM) {
            std::iota(placements.begin(), placements.end(), 1);
            for (int i = static_cast<int>(placements.size() - 1); i > 0; --i) {
                std::uniform_int_distribution<int> dist(0, i);
                int j = dist(app_ptr->Rand());
                std::swap(placements[i], placements[j]);
            }
        } else if (players[0].GetVariableAccount().info.GetPlayOption().GetPlacement(LEFT) == MIRROR) {
            std::iota(placements.rbegin(), placements.rend(), 1);
        }
    }
    for (auto& note : score.notes) {
        auto lane = note->lane;
        assert(lane < MAX_LANE);
        if (players[0].GetVariableAccount().info.GetPlayOption().GetPlacement(LEFT) == RANDOM || players[0].GetVariableAccount().info.GetPlayOption().GetPlacement(LEFT) == MIRROR) {
            if (1 <= lane && lane <= 7) {
                note.get()->lane = placements[lane - 1];
                lane_timelines[note.get()->lane].emplace_back(note.get());
            } else {
                lane_timelines[lane].emplace_back(note.get());
            }
        } else {
            lane_timelines[lane].emplace_back(note.get());
        }
    }
}

void ScorePlayer::Init() {
    LoadBms(app_ptr->GetScoreFilePath().string(), &score);
    SetLaneTimelines();
    wav_manager.Init();
    result = Result();
}

void ScorePlayer::Start() {
    start_ms = TimeManager::GetRealtime();
}

std::unique_ptr<Note> tmp_note(new Note());

ScorePlayer::State ScorePlayer::Update() {
    play_ms = GetPlayMs();
    last_play_ms = play_ms - std::min(play_ms, delta_ms);
    now_pulse = score.MsToPulse(play_ms);
    last_pulse = score.MsToPulse(last_play_ms);

    wav_manager.Update();

    if (players[0].GetVariableAccount().info.GetPlayOption().GetAssistType() == AUTO_PLAY) {
        JudgeAuto();
    } else {
        Judge();
    }

    if (score.info.end_pulse < now_pulse && wav_manager.Empty())
        return State::FINISH;
    else
        return State::CONTINUE;
}

const char* key_str[] = {
    "",       // lane 0 (BGM lane)
    "1pKey1", // lane 1
    "1pKey2", // lane 2
    "1pKey3", // lane 3
    "1pKey4", // lane 4
    "1pKey5", // lane 5
    "1pKey6", // lane 6
    "1pKey7", // lane 7

    "1pScr", // lane 8

    "2pKey1", // lane 9
    "2pKey2", // lane 10
    "2pKey3", // lane 11
    "2pKey4", // lane 12
    "2pKey5", // lane 13
    "2pKey6", // lane 14
    "2pKey7", // lane 15

    "2pScr", // lane 16

    "", // lane 16 (reserved)
    "", // lane 17 (reserved)
    "", // lane 18 (reserved)
    "", // lane 19 (reserved)
};

void ScorePlayer::JudgeAuto() {
    tmp_note->pulse = last_pulse;
    auto begin = score.notes.begin();
    auto end = score.notes.end();
    auto note_it = std::upper_bound(begin, end, tmp_note, ptr_less<Note>());
    for (; note_it != end; ++note_it) {
        Note& note = *(*note_it);
        if (now_pulse < note.pulse)
            break;
        wav_manager.PlayWav(&note);
        if (note.num) {
            result.Push(LaneToSide(note.lane), play_ms, Judge::PGREAT, false);
        }
    }
}

void ScorePlayer::Judge() {
    // lane 0 is BGM
    std::vector<Note*>& bgm_timeline = lane_timelines[0];
    tmp_note->pulse = last_pulse;
    auto begin = bgm_timeline.begin();
    auto end = bgm_timeline.end();
    auto note_it = std::upper_bound(begin, end, tmp_note.get(), ptr_less<Note>());
    for (; note_it != end; ++note_it) {
        Note& note = *(*note_it);
        if (now_pulse < note.pulse)
            break;
        wav_manager.PlayWav(&note);
    }

    for (lane_t lane = 1; lane < MAX_LANE; ++lane) {
        auto& lane_timeline = lane_timelines[lane];
        std::vector<JudgeManager::NoteJudge> notejudges;

        if (Note* note = JudgeManager::UpdateLane(lane_timeline, InputManager::GetKeyFuncState(key_str[lane]), play_ms, lane, score.info.mode, score.info.ln_type, &notejudges)) {
            wav_manager.PlayWav(note);
        }
        for (const auto& notejudge : notejudges) {
            Side side = static_cast<Side>(score.info.mode != Mode::POPN_9K && 9 <= lane);
            result.Push(side, play_ms, notejudge.judge, notejudge.cb_flag);
            if (Note* note = notejudge.note) {
                if (notejudge.stop_wav_flag)
                    wav_manager.StopWav(note);
                if (notejudge.judge == POOR && notejudge.cb_flag == false) {
                    ++note->empty_poor_count;
                } else
                    note->judge = notejudge.judge;
            }
        }
    }
}
