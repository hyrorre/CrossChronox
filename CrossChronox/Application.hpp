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
	
	sf::RenderWindow window;
	sf::RenderTexture renderer;
	
	void ParseArgs(int argc, const char *argv[]);
	
	void Init();
	void Quit();
	Application();
public:
	Application(int argc, const char *argv[]);
	~Application();
	
	int Run();
	
	class Application_Init_Failure : public std::runtime_error{
	public:
		Application_Init_Failure(const std::string& msg): std::runtime_error(msg){}
	};
	
	void HandleException(std::exception e);
};

#endif /* Application_hpp */
