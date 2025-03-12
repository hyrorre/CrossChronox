#pragma once

#include "pch.hpp"
#include "Filesystem/Path.hpp"
#include "Filesystem/ScoreDirectoryInfo.hpp"
#include "Scene.hpp"

class SelectMusic : public Scene {
    bool inited = false;
    ScoreDirectoryInfo root;
    ScoreDirectoryInfo* now_directory = &root;

  public:
    void Init();
    void Deinit() {}
    Scene* Update();
    void Draw() const;
};

extern SelectMusic* scene_select_music_ptr;
