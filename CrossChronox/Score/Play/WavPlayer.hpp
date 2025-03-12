﻿#pragma once

#include "pch.hpp"
#include "Score/ScoreData/ScoreData.hpp"

class WavPlayer {
    static const sf::SoundBuffer buf;

    const Note* note_ptr = nullptr;
    // sf::Sound sound;

  public:
    sf::SoundSource::Status GetStatus() const {
        if (note_ptr)
            return sound.getStatus();
        else
            return sf::Sound::Status::Stopped;
    }
    void Pause() {
        sound.pause();
    }
    void Stop() {
        sound.stop();
    }

    const Note* GetNotePtr() const {
        return note_ptr;
    }

    WavPlayer() : sound(buf) {}
    WavPlayer(const Note* note_ptr) : note_ptr(note_ptr), sound(note_ptr->wavbuf_ptr->buf) {
        sound.play();
    }

    void ResetSound(const Note* note_ptr) {
        this->note_ptr = note_ptr;
        sound.setBuffer(note_ptr->wavbuf_ptr->buf);
        sound.play();
    }
};
