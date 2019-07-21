#pragma once

#include <memory>
#include <vector>

/*

  RenderWorld: 0x1439007C8,
  DebugRender0: 0x143B65BC0,
  DebugRender1: 0x143B65BC1,
  DebugRender8: 0x143B65BCC,
  PlayerHide: 0x143B67F5F,
  AllNoUpdateAi: 0x143B67F66,
  AllNoDamage: 0x143B67F62,
  NoGoodsConsume: 0x143b67f59,
  NoResourceItemConsume: 0x143b67f5a,
  Quitout: [ 0x143B55048, 0x23C ]
 */

#define P(type, addr) reinterpret_cast<type*>(addr)

template<typename t> const t* resolve_pointer_chain(std::vector<void*>());

template<typename T>
class PointerChain {
  private:
    std::vector<uint64_t> chain;
  public:
    PointerChain(std::vector<uint64_t> _chain) : chain(_chain) {};
    T* operator() () {
      uint64_t* p = reinterpret_cast<uint64_t*>(chain[0]);
      for (int i = 1; i < chain.size(); i++) {
        p = (uint64_t*)(*p + chain[i]);
      }
      return reinterpret_cast<T*>(p);
    };
};

class MemoryState {
  private:
    static std::unique_ptr<MemoryState> instance;

    uint32_t* RenderWorld = P(uint32_t, 0x1439007C8);
    uint32_t* DebugRender0 = P(uint32_t, 0x143B65BC0);
    uint32_t* DebugRender1 = P(uint32_t, 0x143B65BC0);
    uint32_t* DebugRender8 = P(uint32_t, 0x143B65BCC);
    uint32_t* PlayerHide = P(uint32_t, 0x143B67F5F);
    uint32_t* AllNoUpdateAI = P(uint32_t, 0x143B67F66);
    uint32_t* AllNoDamage = P(uint32_t, 0x143B67F62);
    uint32_t* NoGoodsConsume = P(uint32_t, 0x143b67f59);
    uint32_t* NoResourceItemConsume = P(uint32_t, 0x143b67f5a);
    PointerChain<float> x, y, z;
    PointerChain<uint8_t> Quitout;

    float stored_x;
    float stored_y;
    float stored_z;

  public:
    MemoryState ();

    void save_pos();
    void load_pos();

    bool toggle_collision ();
    bool toggle_stealth ();
    bool toggle_ai ();
    bool toggle_no_damage ();
    bool toggle_consume ();
    void quitout();
};

#undef P