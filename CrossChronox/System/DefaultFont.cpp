﻿//
//  DefaultFont.cpp
//  CrossChronox
//
//  Created by HY_RORRE on 2016/12/24.
//  Copyright © 2016年 hyrorre. All rights reserved.
//

#include "DefaultFont.hpp"
#include "Filesystem/Path.hpp"

sf::Font font_default;

bool TryInitDefaultFont(){
	return font_default.loadFromFile((GetAppdataPath() / "Fonts/kazesawa/Kazesawa-Regular.ttf").string());
}
