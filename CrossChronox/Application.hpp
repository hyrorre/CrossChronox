#pragma once

#include "pch.hpp"
#include "Filesystem/Path.hpp"
#include "System/Setting.hpp"
#include "Scene/SceneManager.hpp"

class Application {
    SceneManager scene_manager;

    sf::RenderWindow window;
    sf::RenderTexture renderer;

    fs::path executable_path;
    fs::path scorefile_path;

    Setting setting;

    std::mt19937 mt_rand = std::mt19937(std::random_device()());

    void ParseArgs(int argc, char* argv[]);

    sf::Font font_default;

    bool TryInitDefaultFont() {
        return font_default.loadFromFile((GetAppdataPath() / "Fonts/kazesawa/Kazesawa-Regular.ttf").string());
    }

    void Quit();
    Application() = delete;

  public:
    Application(int argc, char* argv[]);
    ~Application();

    void Init();
    int Run();

    SceneManager& GetSceneManager() {
        return scene_manager;
    }

    sf::Font GetDefaultFont() {
        return font_default;
    }

    fs::path& GetScoreFilePath() {
        return scorefile_path;
    }
    void SetScoreFilePath(const fs::path& path) {
        scorefile_path = path;
    }

    std::mt19937& Rand() {
        return mt_rand;
    }

    void HandleException(std::exception& e);
};

extern Application* app_ptr;
