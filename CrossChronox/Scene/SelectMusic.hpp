﻿//
//  SelectMusic.hpp
//  CrossChronox
//
//  Created by HY_RORRE on 2016/12/22.
//  Copyright © 2016年 hyrorre. All rights reserved.
//

#ifndef SelectMusic_hpp
#define SelectMusic_hpp

#include "pch.hpp"
#include "Scene.hpp"
#include "Filesystem/ScoreDirectoryInfo.hpp"
#include "Filesystem/Path.hpp"

class SelectMusic : public Scene{
	bool inited = false;
	ScoreDirectoryInfo root;
	ScoreDirectoryInfo* now_directory = &root;
	
public:
	void Init();
	void Deinit(){}
	Scene* Update();
	void Draw(SDL_Renderer* renderer) const;
};

extern SelectMusic* scene_select_music_ptr;

#endif /* SelectMusic_hpp */
