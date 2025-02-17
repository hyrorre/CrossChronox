#include "DefaultFont.hpp"
#include "Filesystem/Path.hpp"

sf::Font font_default;

bool TryInitDefaultFont() {
    return font_default.loadFromFile((GetAppdataPath() / "Fonts/kazesawa/Kazesawa-Regular.ttf").string());
}
