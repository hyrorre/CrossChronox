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

class SelectMusic : public Scene{
public:
	void Init();
	void Deinit();
	Scene* Update();
	void Draw(sf::RenderTarget& render_target) const;
};

#endif /* SelectMusic_hpp */
