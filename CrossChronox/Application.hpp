//
//  Application.hpp
//  CrossChronox
//
//  Created by HY_RORRE on 9/23/16.
//  Copyright © 2016 hyrorre. All rights reserved.
//

#ifndef Application_hpp
#define Application_hpp

#include "pch.hpp"

class Application{
	static const unsigned int w = 800, h = 600, bpp = 32;
	
	QApplication qapp;
	
	sf::RenderWindow window;
	sf::RenderTexture renderer;
	
	void ParseArgs(int argc, char *argv[]);
	
	void Quit();
	Application() = delete;
public:
	Application(int argc, char *argv[]);
	~Application();
	
	void Init();
	int Run();
	
	class Init_Error : public std::runtime_error{
	public:
		Init_Error(const std::string& msg): std::runtime_error(msg){}
	};
	void HandleException(std::exception& e);
};


#endif /* Application_hpp */
