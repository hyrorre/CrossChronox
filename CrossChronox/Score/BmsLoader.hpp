//
//  BmsLoader.hpp
//  CrossChronox
//
//  Created by HY_RORRE on 10/7/16.
//  Copyright © 2016 hyrorre. All rights reserved.
//

#ifndef BmsLoader_hpp
#define BmsLoader_hpp

#include "pch.hpp"
#include "ScoreData.hpp"

class BmsLoader{
public:
	static bool Load(const std::string& path, ScoreData* out);
	
	//exceptions
	class LoadError: public std::runtime_error{
	public:
		LoadError(const std::string& msg): std::runtime_error(msg){}
		virtual ~LoadError(){}
	};
	class OpenError: public LoadError{
	public:
		OpenError(const std::string& msg): LoadError(msg){}
		virtual ~OpenError(){}
	};
	class ParseError: public LoadError{
	public:
		ParseError(const std::string& msg): LoadError(msg){}
		virtual ~ParseError(){}
	};
};

#endif /* BmsLoader_hpp */
