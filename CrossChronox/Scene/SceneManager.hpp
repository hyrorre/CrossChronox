#pragma once

#include "pch.hpp"
#include "Scene.hpp"

namespace SceneManager {
    enum State {
        FINISH,
        CONTINUE
    };
    void Init();
    State Update();
    void Draw(SDL_Renderer* render_target);
    void Deinit();
}; // namespace SceneManager
