//
//  ScoreInfoBase.hpp
//  CrossChronox
//
//  Created by HY_RORRE on 2016/12/22.
//  Copyright © 2016年 hyrorre. All rights reserved.
//

#ifndef ScoreInfoBase_hpp
#define ScoreInfoBase_hpp

#include "pch.hpp"

class ScoreInfoBase{
protected:
public:
	std::string GetTitleSubtitle() const;
	fs::path path;
	
	ScoreInfoBase(){}
	ScoreInfoBase(fs::path path): path(path){}
};

#endif /* ScoreInfoBase_hpp */
