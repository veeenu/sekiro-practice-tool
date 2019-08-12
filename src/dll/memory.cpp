#include "memory.h"

template<typename T>
inline bool toggle_bit(T* ptr, uint8_t bit = 0) {
  T mask = (1 << bit);
  if (*ptr & mask) {
    *ptr = (*ptr) & ~mask;
    return false;
  } else {
    *ptr = (*ptr) | mask;
    return true;
  }
}

std::unique_ptr<MemoryState> MemoryState::instance;
MemoryState::MemoryState():
  x(PointerChain<float>(std::vector<uint64_t> { 0x143B67DF0, 0x48, 0x28, 0x80 })),
  y(PointerChain<float>(std::vector<uint64_t> { 0x143B67DF0, 0x48, 0x28, 0x84 })),
  z(PointerChain<float>(std::vector<uint64_t> { 0x143B67DF0, 0x48, 0x28, 0x88 })),
  Quitout(PointerChain<uint8_t>(std::vector<uint64_t> { 0x143B55048, 0x23C }))
{}

void MemoryState::save_pos () {
  stored_x = *(x());
  stored_y = *(y());
  stored_z = *(z());
}

void MemoryState::load_pos () {
  *(x()) = stored_x;
  *(y()) = stored_y;
  *(z()) = stored_z;
}

bool MemoryState::toggle_collision () {
  uint32_t s1 = *RenderWorld;
  uint32_t s2 = *DebugRender0;
  uint32_t s3 = *DebugRender8;

  if (s1 & 1) {
    *RenderWorld = s1 & ~1;
    *DebugRender0 = s2 | 1;
    *DebugRender8 = s3 | 1;
    return true;
  } else {
    *RenderWorld = s1 | 1;
    *DebugRender0 = s2 & ~1;
    *DebugRender8 = s3 & ~1;
    return false;
  }

}

bool MemoryState::toggle_stealth () {
  return toggle_bit(PlayerHide); //(*PlayerHide = !(*PlayerHide));
}

bool MemoryState::toggle_ai () {
  return toggle_bit(AllNoUpdateAI); //(*AllNoUpdateAI = !(*AllNoUpdateAI));
}

bool MemoryState::toggle_no_damage () {
  return toggle_bit(AllNoDamage); // (*AllNoDamage = !(*AllNoDamage));
}

bool MemoryState::toggle_consume () {
  return toggle_bit(NoGoodsConsume); // (*NoGoodsConsume = !(*NoGoodsConsume));
}

void MemoryState::quitout () {
  *Quitout() = 1;
}

std::tuple<float, float, float, float, float, float> MemoryState::get_position() const {
  float *px = x(), *py = y(), *pz = z();
  return std::make_tuple(
    px == nullptr ? 0.f : *px,
    py == nullptr ? 0.f : *py,
    pz == nullptr ? 0.f : *pz,
    stored_x, stored_y, stored_z
  );
};