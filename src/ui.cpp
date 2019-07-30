#include "ui.h"
#include "config.h"
#include "tinyformat.h"
#include <windows.h>

Command::Command(const callback& _fn, const std::string& _label, const uint64_t _key) : fn(_fn), label(_label), key(_key) {}

void Command::set_key(const uint64_t k) {
  key = k;
}

const uint64_t Command::get_key() const {
  return key;
}

const std::string& Command::get_label() const {
  return label;
}

void Command::operator() () {
  std::cout << "Executing command " << label << std::endl;
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
  }, "show", cfg["show"]));

  commands.push_back(Command([this]() {
    collision = state.toggle_collision();
  }, "collision", cfg["collision"]));

  commands.push_back(Command([this]() {
    stealth = state.toggle_stealth();
  }, "stealth", cfg["stealth"]));

  commands.push_back(Command([this]() {
    ai = state.toggle_ai();
  }, "ai", cfg["ai"]));

  commands.push_back(Command([this]() {
    no_damage = state.toggle_no_damage();
  }, "no_damage", cfg["no_damage"]));

  commands.push_back(Command([this]() {
    consume = state.toggle_consume();
  }, "consume", cfg["consume"]));

  commands.push_back(Command([this]() {
    state.save_pos();
  }, "save_pos", cfg["save_pos"]));

  commands.push_back(Command([this]() {
    state.load_pos();
  }, "load_pos", cfg["load_pos"]));

  commands.push_back(Command([this]() {
    state.quitout();
  }, "quitout", cfg["quitout"]));
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

  if (show_window) {
    if (ImGui::Begin("Practice tool")) {
      ImGui::SetWindowPos(ImVec2(25., 25.));
      ImGui::SetWindowSize(ImVec2(400., 300.));
      ImGui::Checkbox(tfm::format("Collision Meshes (%s)", cfg.repr("collision")).c_str(), &collision);
      ImGui::Checkbox(tfm::format("Stealth (%s)", cfg.repr("stealth")).c_str(), &stealth);
      ImGui::Checkbox(tfm::format("AI Freeze (%s)", cfg.repr("ai")).c_str(), &ai);
      ImGui::Checkbox(tfm::format("No Damage (%s)", cfg.repr("no_damage")).c_str(), &no_damage);
      ImGui::Checkbox(tfm::format("Consume (%s)", cfg.repr("consume")).c_str(), &consume);
      //ImGui::Text(tfm::format("Load Position (%s)", cfg.repr("load_pos")).c_str());
      //ImGui::Text(tfm::format("Save Position (%s)", cfg.repr("save_pos")).c_str());
      auto pos = state.get_position();
      ImGui::Text(tfm::format("Current Position: (Load %s | Save %s)\n(% 12.5f, % 12.5f, % 12.5f)", 
        cfg.repr("load_pos"), cfg.repr("save_pos"),
        std::get<0>(pos), std::get<1>(pos), std::get<2>(pos)).c_str());
      ImGui::Text(tfm::format("  Saved Position:\n(% 12.5f, % 12.5f, % 12.5f)",
        std::get<3>(pos), std::get<4>(pos), std::get<5>(pos)).c_str());
      ImGui::Text(tfm::format("Quitout (%s)", cfg.repr("quitout")).c_str());
    }
    ImGui::End();
  }

  for (int i = 0; i < 512; i++) {
    prevKeys[i] = io.KeysDown[i];
  }

}

bool UI::is_keyup(const ImGuiIO& io, int k) {
  return !io.KeysDown[k] && prevKeys[k];
}
