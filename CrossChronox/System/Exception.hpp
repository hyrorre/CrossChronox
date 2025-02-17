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

class InitError : public std::runtime_error{
public:
	InitError(const std::string& msg): std::runtime_error(msg){}
	virtual ~InitError(){}
};

#endif /* Exception_hpp */
