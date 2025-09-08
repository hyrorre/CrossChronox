#pragma once

#include "pch.hpp"

class Application {
    SDL_Window* window = NULL;
    SDL_Renderer* renderer = NULL;

    static fs::path executable_path;
    static fs::path scorefile_path;

  public:
    SDL_AppResult Init(int argc, char* argv[]);
    SDL_AppResult Event(SDL_Event* event);
    SDL_AppResult Run();
    void Quit();

    static fs::path& GetScoreFilePath() {
        return scorefile_path;
    }
    static void SetScoreFilePath(const fs::path& path) {
        scorefile_path = path;
    }

    // void HandleException(std::exception& e);
};

extern Application app;
