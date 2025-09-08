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

SDL_AppResult Application::Init(int argc, char* argv[]) {
    if (argc > 0) {
        executable_path = argv[0];
fs::current_path(fs::path(executable_path).parent_path());
    }
    if (argc > 1) {
        scorefile_path = argv[1];
        if (!fs::exists(scorefile_path) || !fs::is_regular_file(scorefile_path)) {
            scorefile_path.clear();
        }
    }
    // set locale
    setlocale(LC_ALL, "");

    if (!SDL_Init(SDL_INIT_VIDEO | SDL_INIT_GAMEPAD)) {
        SDL_Log("Couldn't initialize SDL: %s", SDL_GetError());
        return SDL_APP_FAILURE;
    }

    if (!TTF_Init()) {
        SDL_Log("Couldn't initialize SDL_ttf: %s", SDL_GetError());
        return SDL_APP_FAILURE;
    }
    
    if (!(window = SDL_CreateWindow("CrossChronox", 1920, 1080, SDL_WINDOW_HIGH_PIXEL_DENSITY))) {
        SDL_Log("Couldn't create window: %s", SDL_GetError());
        return SDL_APP_FAILURE;
    }
    
    if (!(renderer = SDL_CreateRenderer(window, nullptr))) {
        SDL_Log("Couldn't create renderer: %s", SDL_GetError());
        return SDL_APP_FAILURE;
    }
    
    fs::path appdata_path = GetAppdataPath();

    InputManager::LoadConfig((appdata_path / "Config/KeyConfig.toml").string());
    InputManager::SetMode("Beat");

    SceneManager::Init(renderer);

    return SDL_APP_CONTINUE;
}

SDL_AppResult Application::Event(SDL_Event* event) {
    if (event->type == SDL_EVENT_QUIT) {
        return SDL_APP_SUCCESS; /* end the program, reporting success to the OS. */
    }
    if (event->type == SDL_EVENT_KEY_DOWN) {
        // std::cout << event->key.scancode << std::endl;
    }
    return SDL_APP_CONTINUE; /* carry on with the program! */
}

SDL_AppResult Application::Run() {
    TimeManager::Update();
    InputManager::Update();

    SceneManager::Update(renderer);
    SceneManager::Draw(renderer);

    SDL_RenderPresent(renderer);
    SDL_RenderClear(renderer);
    return SDL_APP_CONTINUE;
}

void Application::Quit() {
    TTF_Quit();
    SDL_Quit();
}
