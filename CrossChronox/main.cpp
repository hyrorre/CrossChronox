//
//  main.cpp
//  CrossChronox
//
//  Created by HY_RORRE on 9/23/16.
//  Copyright © 2016 hyrorre. All rights reserved.
//

#include "Application.hpp"
#include "Filesystem/Path.hpp"

int main(int argc, char * argv[]){
	Application app(argc, argv);
	try{
		app.Init();
		return app.Run();
	}
	catch(std::exception& e){
		app.HandleException(e);
		return EXIT_FAILURE;
	}
}
