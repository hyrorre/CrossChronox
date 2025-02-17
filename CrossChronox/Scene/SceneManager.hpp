#ifndef SceneManager_hpp
#define SceneManager_hpp

#include "pch.hpp"
#include "Scene.hpp"

namespace SceneManager{
	enum State{
		FINISH,
		CONTINUE
	};
	void Init();
	State Update();
	void Draw(sf::RenderTarget& render_target);
	void Deinit();
};

#endif /* SceneManager_hpp */
