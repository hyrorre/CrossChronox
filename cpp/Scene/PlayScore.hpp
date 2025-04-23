#pragma once

#include "pch.hpp"
#include "Scene.hpp"
#include "Score/Play/ScorePlayer.hpp"

class PlayScore : public Scene {
  public:
    void Init(SDL_Renderer* renderer);
    void Deinit();
    Scene* Update();
    void Draw(SDL_Renderer* renderer) const;
};

extern PlayScore* scene_play_score_ptr;
