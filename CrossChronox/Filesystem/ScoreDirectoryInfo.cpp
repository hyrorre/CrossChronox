//
//  ScoreDirectoryInfo.cpp
//  CrossChronox
//
//  Created by HY_RORRE on 2016/12/22.
//  Copyright © 2016年 hyrorre. All rights reserved.
//

#include "ScoreDirectoryInfo.hpp"
#include "Application.hpp"

void ScoreDirectoryInfo::Decide() const{
	Application::SetScoreFilePath(At(0)->path);
}
