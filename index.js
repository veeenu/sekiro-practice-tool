const { dialog, app, BrowserWindow, globalShortcut, ipcMain } = require('electron')
const amnesiaclapp = require('amnesiaclapp')
const path = require('path')

let url
if (process.env.NODE_ENV === 'dev') {
  url = 'http://localhost:8080/'
} else {
  url = `file://${process.cwd()}/resources/app/dist/index.html`
}

const ptrs = {
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
}

function SekiroProcessComm () {
  const p = new amnesiaclapp.Process('sekiro.exe')

  var position = { x: null, y: null, z: null }

  return {
    checkReattach () {
      if (!p.attached()) {
        p.reattach()
      }
    },

    toggleCollisionMeshes () {
      let w = p.readU32(ptrs.RenderWorld)
      let r0 = p.readU32(ptrs.DebugRender0)
      let r8 = p.readU32(ptrs.DebugRender8)

      if (w & 1) {
        p.writeU32(ptrs.RenderWorld, w & ~1)
        p.writeU32(ptrs.DebugRender0, r0 | 1)
        p.writeU32(ptrs.DebugRender8, r8 | 1)
        return true
      } else {
        p.writeU32(ptrs.RenderWorld, w | 1)
        p.writeU32(ptrs.DebugRender0, r0 & ~1)
        p.writeU32(ptrs.DebugRender8, r8 & ~1)
        return false
      }
    },

    toggleStealth () {
      let h = p.readU32(ptrs.PlayerHide)
      if (h & 1) {
        p.writeU32(ptrs.PlayerHide, h & ~1)
        return false
      } else {
        p.writeU32(ptrs.PlayerHide, h | 1)
        return true
      }
    },

    toggleAI () {
      let h = p.readU32(ptrs.AllNoUpdateAi)
      if (h & 1) {
        p.writeU32(ptrs.AllNoUpdateAi, h & ~1)
        return false
      } else {
        p.writeU32(ptrs.AllNoUpdateAi, h | 1)
        return true
      }
    },

    toggleNoDamage () {
      let h = p.readU32(ptrs.AllNoDamage)
      if (h & 1) {
        p.writeU32(ptrs.AllNoDamage, h & ~1)
        return false
      } else {
        p.writeU32(ptrs.AllNoDamage, h | 1)
        return true
      }
    },

    toggleConsume () {
      let h = p.readU32(ptrs.NoGoodsConsume)
      if (h & 1) {
        p.writeU32(ptrs.NoGoodsConsume, h & ~1)
        return false
      } else {
        p.writeU32(ptrs.NoGoodsConsume, h | 1)
        return true
      }
    },

    quitout () {
      let qp = p.pointer(ptrs.Quitout)
      p.writeU32(qp, 1)
    },

    savePosition () {
      let xp = p.pointer([0x143B67DF0, 0x48, 0x28, 0x80])
      let yp = xp + 4
      let zp = yp + 4

      position.x = p.readF(xp)
      position.y = p.readF(yp)
      position.z = p.readF(zp)
    },

    loadPosition () {
      let xp = p.pointer([0x143B67DF0, 0x48, 0x28, 0x80])
      let yp = xp + 4
      let zp = yp + 4

      if (position.x != null) {
        p.writeF(xp, position.x)
        p.writeF(yp, position.y)
        p.writeF(zp, position.z)
      }
    }
  }
}

let spcomm

app.on('ready', () => {
   let win = new BrowserWindow({
    width: 300,
    height: 350,
    resizable: (process.env.NODE_ENV == 'dev' ? true : false),
    webPreferences: {
      nodeIntegration: true,
      preload: __dirname + '/preload.js'
    },
    icon: path.join(__dirname, "public/sidekiro.png")
  })
  win.loadURL(url)
  win.setMenuBarVisibility(false)
  if (process.env.NODE_ENV === 'dev') {
    win.openDevTools({ mode: 'detach' })
  }

  for(let i of [ 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12 ]) {
    globalShortcut.register('F' + i, () => {
      win.webContents.sendInputEvent({ type: 'keyUp', keyCode: 'F'+i, __special: true })
    })

  }

  try {
    spcomm = new SekiroProcessComm()

    ipcMain.on('execute', (evt, arg) => {
      if (arg in spcomm) {
        spcomm.checkReattach()
        let ret = spcomm[arg]()
        if (ret === false || ret === true) {
          evt.reply('flag-change', { flag: arg, value: ret })
        }
      }
    })
  } catch (err) {
    dialog.showMessageBox({
      title: 'Error',
      message: 'Could not open sekiro.exe (' + err.toString() + ').'
    })

    window.close()
    app.exit()
  }
})