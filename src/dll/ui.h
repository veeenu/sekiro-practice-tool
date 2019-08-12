#pragma once

#include <functional>
#include <vector>
#include <string>
#include <memory>
#include <imgui.h>
#include <tinyformat.h>

#include "memory.h"

typedef std::function<void(void)> callback;

class Command {
  private:
    callback fn;
    std::string label;
    uint64_t key;

  public:
    Command(const callback& _fn, const std::string& _label, uint64_t _key);
    void set_key(const uint64_t k);
    const uint64_t get_key() const;
    const std::string& get_label() const;
    void operator() ();
};

class UI {
  private:
    static std::unique_ptr<UI> instance;
    UI();

    std::vector<bool> prevKeys;
    std::vector<Command> commands;

    bool show_window = false;

    bool collision = false;
    bool stealth = false;
    bool ai = false;
    bool no_damage = false;
    bool consume = false;

    MemoryState state;

  public:
    static UI& const Instance();
    void Render();
    bool is_keyup(const ImGuiIO& io, int k);
};
