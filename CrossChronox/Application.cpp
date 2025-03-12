#include "Application.hpp"
#include "Filesystem/Path.hpp"
#include "Scene/SceneManager.hpp"
#include "Score/Load/BmsLoader.hpp"
#include "Score/Play/ScorePlayer.hpp"
#include "System/InputManager.hpp"
#include "System/Setting.hpp"

Application app;
fs::path Application::scorefile_path;
fs::path Application::executable_path;

void Application::ParseArgs(int argc, char* argv[]) {
    if (argc > 0)
        executable_path = argv[0];
    if (argc > 1) {
        scorefile_path = argv[1];
        if (!fs::exists(scorefile_path) || !fs::is_regular_file(scorefile_path)) {
            scorefile_path.clear();
        }
    }
}

void Application::Init() {
    // set locale
    setlocale(LC_ALL, "");

    InputManager::LoadConfig((GetAppdataPath() / "Config/KeyConfig.toml").string());
    InputManager::SetMode("Beat");

    SceneManager::Init();
}

void Application::Run() {
    TimeManager::Update();
    InputManager::Update();

    SceneManager::Update();
    SceneManager::Draw();

    EndDrawing();
}

void Application::Quit() {
}
