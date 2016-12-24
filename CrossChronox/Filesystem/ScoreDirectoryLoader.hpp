//
//  ScoreDirectoryLoader.hpp
//  CrossChronox
//
//  Created by HY_RORRE on 2016/12/24.
//  Copyright © 2016年 hyrorre. All rights reserved.
//

#ifndef ScoreDirectoryLoader_hpp
#define ScoreDirectoryLoader_hpp

#include "pch.hpp"
#include "ScoreDirectoryInfo.hpp"
#include "ScoreData.hpp"

class ScoreDirectoryLoader{
	ScoreData tmp_data;
	void LoadScores(const fs::path& path, ScoreDirectoryInfo* out);
public:
	void Load(const fs::path& path, ScoreDirectoryInfo* out);
};

#endif /* ScoreDirectoryLoader_hpp */
