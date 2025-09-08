#include "Path.hpp"

const fs::path GetAppdataPath() {
    // TODO: set resource path (support external DerivedData)
    fs::path path = fs::current_path();
    do {
        if (fs::exists(path / "CrossChronoxData")) {
            return path / "CrossChronoxData";
        }
        path = path.parent_path();
    } while(path.has_parent_path());
    throw std::runtime_error("\"CrossChronoxData\" folder was not found.");
}
