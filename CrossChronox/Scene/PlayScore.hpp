#pragma once

#include "pch.hpp"
#include "Scene.hpp"
#include "Score/Play/ScorePlayer.hpp"

class PlayScore : public Scene {
  public:
    void Init();
    void Deinit();
    Scene* Update();
    void Draw() const;
};

extern PlayScore* scene_play_score_ptr;
