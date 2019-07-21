#include "ui.h"
#include "config.h"
#include "tinyformat.h"
#include <windows.h>

Command::Command(const callback& _fn, const uint64_t _key) : fn(_fn), key(_key) {}

void Command::set_key(const uint64_t k) {
  key = k;
}

const uint64_t Command::get_key() const {
  return key;
}

void Command::operator() () {
  fn();
}

std::unique_ptr<UI> UI::instance;
UI::UI () {
  prevKeys.reserve(512);

  for (short i = 0; i < 512; i++) {
    prevKeys[i] = 0;
  }

  auto cfg = Config::Instance();

  // Toggle
  commands.push_back(Command([this]() {
    show_window = !show_window;
  }, cfg["show"]));

  commands.push_back(Command([this]() {
    collision = state.toggle_collision();
  }, cfg["collision"]));

  commands.push_back(Command([this]() {
    stealth = state.toggle_stealth();
  }, cfg["stealth"]));

  commands.push_back(Command([this]() {
    ai = state.toggle_ai();
  }, cfg["ai"]));

  commands.push_back(Command([this]() {
    no_damage = state.toggle_no_damage();
  }, cfg["no_damage"]));

  commands.push_back(Command([this]() {
    consume = state.toggle_consume();
  }, cfg["consume"]));

  commands.push_back(Command([this]() {
    state.save_pos();
  }, cfg["save_pos"]));

  commands.push_back(Command([this]() {
    state.load_pos();
  }, cfg["load_pos"]));

  commands.push_back(Command([this]() {
    state.quitout();
  }, cfg["quitout"]));
}

UI& const UI::Instance () {
  if (!instance) {
    instance.reset(new UI());
  }
  return *(instance.get());
}

void UI::Render() {
  ImGuiIO& io = ImGui::GetIO();
  auto& cfg = Config::Instance();

  for (auto& i : commands) {
    if (is_keyup(io, i.get_key())) {
      i();
    }
  }

  ImGui::SetNextWindowPos(ImVec2(0.025, 0.025));
  ImGui::SetNextWindowSize(ImVec2(0.3, 0.3));

  if (show_window) {
    ImGui::Checkbox(tfm::format("Collision Meshes (%s)", cfg.repr("collision")).c_str(), &collision);
    ImGui::Checkbox(tfm::format("Stealth (%s)", cfg.repr("stealth")).c_str(), &stealth);
    ImGui::Checkbox(tfm::format("AI Freeze (%s)", cfg.repr("ai")).c_str(), &ai);
    ImGui::Checkbox(tfm::format("No Damage (%s)", cfg.repr("no_damage")).c_str(), &no_damage);
    ImGui::Checkbox(tfm::format("Consume (%s)", cfg.repr("consume")).c_str(), &consume);
    ImGui::Text(tfm::format("Quitout (%s)", cfg.repr("quitout")).c_str());
    ImGui::Text(tfm::format("Load Position (%s)", cfg.repr("load_pos")).c_str());
    ImGui::Text(tfm::format("Save Position (%s)", cfg.repr("save_pos")).c_str());
  }

  for (int i = 0; i < 512; i++) {
    prevKeys[i] = io.KeysDown[i];
  }

}

bool UI::is_keyup(const ImGuiIO& io, int k) {
  return !io.KeysDown[k] && prevKeys[k];
}
