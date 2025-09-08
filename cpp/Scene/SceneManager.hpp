#pragma once

#include "pch.hpp"
#include "Scene.hpp"

namespace SceneManager {
    enum State {
        FINISH,
        CONTINUE
    };
    void Init(SDL_Renderer* renderer);
    State Update(SDL_Renderer* renderer);
    void Draw(SDL_Renderer* renderer);
    void Deinit();
}; // namespace SceneManager
