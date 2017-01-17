//
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
#include "ScoreDirectoryInfo.hpp"
#include "Path.hpp"

class SelectMusic : public Scene{
	bool inited = false;
	ScoreDirectoryInfo root;
	ScoreDirectoryInfo* now_directory = &root;
	//int cursor = 0;
public:
	void Init();
	void Deinit(){}
	Scene* Update();
	void Draw(sf::RenderTarget& render_target) const;
};

extern SelectMusic* scene_select_music_ptr;

#endif /* SelectMusic_hpp */
