#include "DefaultFont.hpp"
#include "Filesystem/Path.hpp"

sf::Font font_default((GetAppdataPath() / "Fonts/kazesawa/Kazesawa-Regular.ttf").string());
