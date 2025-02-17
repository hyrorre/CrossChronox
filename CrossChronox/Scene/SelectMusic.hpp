#pragma once

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
	void Draw(sf::RenderTarget& render_target) const;
};

extern SelectMusic* scene_select_music_ptr;
