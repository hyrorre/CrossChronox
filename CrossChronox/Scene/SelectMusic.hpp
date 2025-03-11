#pragma once

#include "pch.hpp"
#include "Filesystem/Path.hpp"
#include "Filesystem/ScoreDirectoryInfo.hpp"
#include "Scene.hpp"

class SelectMusic : public Scene {
    bool inited = false;
    ScoreDirectoryInfo root;
    ScoreDirectoryInfo* now_directory = &root;

    sf::Text text_songlist;
    sf::Text text_info;

    std::wstring str_songlist;

  public:
    void Init();
    void Deinit() {}
    Scene* Update();
    void Draw(sf::RenderTarget& render_target);
};
