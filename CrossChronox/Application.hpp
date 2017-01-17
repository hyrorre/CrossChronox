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
	//QApplication qapp;
	
	sf::RenderWindow window;
	sf::RenderTexture renderer;
	
	fs::path executable_path;
	static fs::path scorefile_path;
	
	void ParseArgs(int argc, char *argv[]);
	
	void Quit();
	Application() = delete;
public:
	Application(int argc, char *argv[]);
	~Application();
	
	void Init();
	int Run();
	
	static fs::path& GetScoreFilePath(){
		return scorefile_path;
	}
	static void SetScoreFilePath(const fs::path& path){
		scorefile_path = path;
	}
	
	class InitError : public std::runtime_error{
	public:
		InitError(const std::string& msg): std::runtime_error(msg){}
		virtual ~InitError(){}
	};
	void HandleException(std::exception& e);
};


#endif /* Application_hpp */
