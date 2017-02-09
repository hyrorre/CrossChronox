//
//  PlayScore.hpp
//  CrossChronox
//
//  Created by HY_RORRE on 2016/10/10.
//  Copyright © 2016年 hyrorre. All rights reserved.
//

#ifndef PlayScore_hpp
#define PlayScore_hpp

#include "pch.hpp"
#include "Scene.hpp"
#include "Score/Play/ScorePlayer.hpp"

class PlayScore : public Scene{
	static const int MAX_PLAYER = 1;
	std::array<ScorePlayer, MAX_PLAYER> players;
	
public:
	void Init();
	void Deinit();
	Scene* Update();
	void Draw(sf::RenderTarget& render_target) const;
};

extern PlayScore* scene_play_score_ptr;

#endif /* PlayScore_hpp */
