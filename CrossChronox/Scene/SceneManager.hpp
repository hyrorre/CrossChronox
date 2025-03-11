#pragma once

#include "pch.hpp"
#include "Scene.hpp"
#include "Scene/PlayScore.hpp"
#include "Scene/SelectMusic.hpp"

class SceneManager {
    Scene* now_scene;

  public:
    PlayScore play_score;
    SelectMusic select_music;

    enum State {
        FINISH,
        CONTINUE
    };

    SceneManager() {
        now_scene = &select_music;
        // Init();
    }

    SceneManager(Scene* scene) : now_scene(scene) {
        // Init();
    }

    ~SceneManager() {
        // Deinit();
    }

    void Init() {
        now_scene->Init();
    }

    State Update() {
        Scene* next_scene = now_scene->Update();
        if (now_scene != next_scene) {
            now_scene->Deinit();
            if (next_scene == nullptr) {
                return FINISH;
            }
            now_scene = next_scene;
            next_scene->Init();
            next_scene->Update();
        }
        return CONTINUE;
    }

    void Draw(sf::RenderTarget& render_target) {
        if (now_scene)
            now_scene->Draw(render_target);
    }

    void Deinit() {
        if (now_scene) {
            now_scene->Deinit();
            now_scene = nullptr;
        }
    }
};
