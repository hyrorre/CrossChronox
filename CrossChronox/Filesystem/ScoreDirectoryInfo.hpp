//
//  ScoreDirectoryInfo.hpp
//  CrossChronox
//
//  Created by HY_RORRE on 2016/12/22.
//  Copyright © 2016年 hyrorre. All rights reserved.
//

#ifndef ScoreDirectoryInfo_hpp
#define ScoreDirectoryInfo_hpp

#include "pch.hpp"
#include "ScoreInfoBase.hpp"

class ScoreDirectoryInfo : public ScoreInfoBase{
	using cursor_t = int;
	
public:
	std::string title;
	std::vector<std::unique_ptr<ScoreInfoBase>> children;
	cursor_t cursor;
};


#endif /* ScoreDirectoryInfo_hpp */
