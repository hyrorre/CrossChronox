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

class BmsLoader: private boost::noncopyable{
	std::unordered_map<int, double> exbpm;
	std::list<int> lnobj;
	ScoreData* out = nullptr;
	const char* nowline = nullptr;
	int line_num = 0; //行番号
	
	int random_num = 0;
	bool parse_nextline_flag = true;
	
	const char* GetArg();
	int GetIndex();
	bool ParseLine();
	bool TryParseHeaderLine();
	bool TryParseObjLine();
	bool InitCommands();
	bool Init(ScoreData* out);
public:
	BmsLoader();
	bool Load(const std::string& path, ScoreData* out);
	
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
