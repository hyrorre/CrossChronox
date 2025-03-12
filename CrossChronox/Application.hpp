﻿#pragma once

#include "pch.hpp"

class Application {
    static fs::path executable_path;
    static fs::path scorefile_path;

    void ParseArgs(int argc, char* argv[]);

  public:
    void Init();
    void Run();
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
