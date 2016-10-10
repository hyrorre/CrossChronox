//
//  ScoreParser.hpp
//  CrossChronox
//
//  Created by HY_RORRE on 10/7/16.
//  Copyright © 2016 hyrorre. All rights reserved.
//

#ifndef ScoreParser_hpp
#define ScoreParser_hpp

#include "pch.hpp"

class ScoreParser{
public:
	virtual bool parse(const boost::string_ref& path) = 0;
};

#endif /* ScoreParser_hpp */
