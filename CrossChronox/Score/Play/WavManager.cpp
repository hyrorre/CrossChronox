#include "WavManager.hpp"

void WavManager::Update() {
}

void WavManager::PlayWav(const Note* note) {
    if (note && note->wavbuf_ptr) {
        for (auto& player : players) {
            // if (player.GetStatus() == sf::Sound::Status::Stopped) {
            //     player.ResetSound(note);
            //     break;
            // }
        }
    }
}

void WavManager::StopWav(const Note* note) {
    // playersの線形探索により、引数とplayerが持つnote_ptrが一致するものを探し、そのWavをStop
    for (auto& player : players) {
        if (player.GetNotePtr() == note) {
            player.Stop();
            return;
        }
    }
}
