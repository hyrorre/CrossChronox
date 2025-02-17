#ifndef Application_hpp
#define Application_hpp

#include "pch.hpp"

class Application{
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
	
	void HandleException(std::exception& e);
};


#endif /* Application_hpp */
