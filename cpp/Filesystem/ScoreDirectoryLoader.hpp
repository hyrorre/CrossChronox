#pragma once

#include "pch.hpp"
#include "Score/ScoreData/ScoreData.hpp"
#include "ScoreDirectoryInfo.hpp"

class ScoreDirectoryLoader {
    ScoreData tmp_data;
    void LoadScores(const fs::path& path, ScoreDirectoryInfo* out);

  public:
    void Load(const fs::path& path, ScoreDirectoryInfo* out);
};
