//
//  main.cpp
//  CrossChronox
//
//  Created by HY_RORRE on 9/23/16.
//  Copyright © 2016 hyrorre. All rights reserved.
//

#include "Application.hpp"

int main(int argc, const char * argv[]) {
	try{
		Application app(argc, argv);
		return app.Run();
	}
	catch(std::exception& e){
		
		return EXIT_FAILURE;
	}
}
