//
//  BmsParser.hpp
//  CrossChronox
//
//  Created by HY_RORRE on 10/7/16.
//  Copyright © 2016 hyrorre. All rights reserved.
//

#ifndef BmsParser_hpp
#define BmsParser_hpp

#include "pch.hpp"
#include "ScoreParser.hpp"

class BmsParser{
public:
	bool parse(const boost::string_ref& path);
};

#endif /* BmsParser_hpp */
