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
    void Draw();
    void Deinit();
}; // namespace SceneManager
