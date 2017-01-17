//
//  Exception.hpp
//  CrossChronox
//
//  Created by HY_RORRE on 2016/11/22.
//  Copyright © 2016 hyrorre. All rights reserved.
//

#ifndef Exception_hpp
#define Exception_hpp

//exceptions
class LoadError : public std::runtime_error{
public:
	LoadError(const std::string& msg): std::runtime_error(msg){}
	virtual ~LoadError(){}
};
class OpenError : public LoadError{
public:
	OpenError(const std::string& msg): LoadError(msg){}
	virtual ~OpenError(){}
};
class ParseError : public LoadError{
public:
	ParseError(const std::string& msg): LoadError(msg){}
	virtual ~ParseError(){}
};

#endif /* Exception_hpp */
