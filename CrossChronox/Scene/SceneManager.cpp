//
//  SceneManager.cpp
//  CrossChronox
//
//  Created by HY_RORRE on 9/24/16.
//  Copyright © 2016 hyrorre. All rights reserved.
//

#include "SceneManager.hpp"

using scene_stack_type = std::stack<std::forward_list<std::unique_ptr<IScene>>>;
scene_stack_type scene_stack;
