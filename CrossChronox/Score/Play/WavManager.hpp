#pragma once

#include "pch.hpp"
#include "Score/ScoreData/ScoreData.hpp"
#include "WavPlayer.hpp"

class WavManager {
    static const int MAX_SOUND = 128;
    using container_t = std::vector<WavPlayer>;
    container_t players;

  public:
    void Update() {}

    void PlayWav(const Note* note) {
        if (note && note->wavbuf_ptr) {
            for (auto& player : players) {
                if (player.GetStatus() == sf::Sound::Status::Stopped) {
                    player.ResetSound(note);
                    break;
                }
            }
        }
    }

    void StopWav(const Note* note) {
        // playersの線形探索により、引数とplayerが持つnote_ptrが一致するものを探し、そのWavをStop
        for (auto& player : players) {
            if (player.GetNotePtr() == note) {
                player.Stop();
                return;
            }
        }
    }

    bool Empty() const {
        for (auto& player : players) {
            if (player.GetStatus() != sf::Sound::Status::Stopped) {
                return false;
            }
        }
        return true;
    }

    void Init() {
        players.resize(MAX_SOUND);
    }
    
    void Clear() {
        players.clear();
    }

    WavManager() {}
};
