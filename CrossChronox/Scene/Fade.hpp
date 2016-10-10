//
//  Fade.hpp
//  CrossChronox
//
//  Created by HY_RORRE on 10/5/16.
//  Copyright © 2016 hyrorre. All rights reserved.
//

#ifndef Fade_hpp
#define Fade_hpp

#include "SceneManager.hpp"

class Fade : public IScene{
protected:
	sf::Clock clock;
public:
	virtual void Update();
	void Render();
	
	virtual ~Fade(){}
};

class FadeIn : public Fade{
public:
	void Update();
};

class FadeOut : public Fade{
public:
	void Update();
};

#endif /* Fade_hpp */
