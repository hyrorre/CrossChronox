//
//  Path.cpp
//  CrossChronox
//
//  Created by HY_RORRE on 2017/01/18.
//  Copyright © 2017年 hyrorre. All rights reserved.
//

#include "Path.hpp"

const fs::path& GetAppdataPath(){
	static fs::path apath;
	if(apath.empty()){
		apath = fs::current_path();
		while(apath.has_parent_path()){
			fs::path tmp_path = apath / "CrossChronoxData";
			if(fs::exists(tmp_path)){
				apath = tmp_path;
				goto ReturnPath;
			}
			apath = apath.parent_path();
		}
		throw InitError("\"CrossChronoxData\" folder was not found.");
	}
ReturnPath:
	return apath;
}
