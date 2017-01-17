#pragma once

#include "Path.hpp"

namespace Path{
	fs::path appdataPath(void){
		fs::path apath = fs::current_path();
		while(apath.has_parent_path()){
			apath = apath.parent_path();
			fs::path tmp_path = apath / "CrossChronoxData";
			if(fs::exists(tmp_path)) return tmp_path;
		}
		throw Path::InitError("\"CrossChronoxData\" folder was not found.");
	}

	fs::path resource_;
	fs::path font_;
	fs::path appdata_;
	fs::path cache_;

	const fs::path& resource = resource_;
	const fs::path& font = font_;
	const fs::path& appdata = appdata_;
	const fs::path& cache = cache_;

	void Init(void){
		appdata_ = appdataPath();
	}
}

