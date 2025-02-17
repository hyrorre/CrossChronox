﻿#pragma once

#include "pch.hpp"
#include "ScoreDirectoryInfo.hpp"
#include "Score/ScoreData/ScoreData.hpp"

class ScoreDirectoryLoader{
	ScoreData tmp_data;
	void LoadScores(const fs::path& path, ScoreDirectoryInfo* out);
public:
	void Load(const fs::path& path, ScoreDirectoryInfo* out);
};
