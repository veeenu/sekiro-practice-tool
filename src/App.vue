<template lang='pug'>
div#app
  button(@click='sendMessage("savePosition")')
    label Save position (F7)
  button(@click='sendMessage("loadPosition")')
    label Load position (F1)
  button(@click='sendMessage("toggleCollisionMeshes")')
    span {{ flags.toggleCollisionMeshes ? "&#9745;&nbsp;" : "&#9744;&nbsp;" }}
    label Toggle collision meshes (F5)
  button(@click='sendMessage("toggleAI")') 
    span {{ flags.toggleAI ? "&#9745;&nbsp;" : "&#9744;&nbsp;" }}
    label Toggle AI (F8)
  button(@click='sendMessage("toggleNoDamage")')
    span {{ flags.toggleNoDamage ? "&#9745;&nbsp;" : "&#9744;&nbsp;" }}
    label Toggle No Damage (F9)
  button(@click='sendMessage("toggleStealth")') 
    span {{ flags.toggleStealth ? "&#9745;&nbsp;" : "&#9744;&nbsp;" }}
    label Toggle Stealth (F11)
  button(@click='sendMessage("toggleConsume")') 
    span {{ flags.toggleConsume ? "&#9745;&nbsp;" : "&#9744;&nbsp;" }}
    label Toggle Consume (F4)
  button(@click='sendMessage("quitout")') 
    label Quitout (F6)
</template>

<script>
const ipcRenderer = window.ipcRenderer

export default {
  name: 'app',
  data () {
    return {
      flags: {
        toggleCollisionMeshes: false,
        toggleAI: false,
        toggleNoDamage: false,
        toggleStealth: false,
        toggleConsume: false
      }
    }
  },
  methods: {
    sendMessage (i) {
      ipcRenderer.send('execute', i)
    },
  },
  mounted () {
    document.body.addEventListener('keyup', evt => {
      evt.stopPropagation()
      switch (evt.code) {
        case 'F1':
          this.sendMessage('loadPosition')
          break;
        case 'F4':
          this.sendMessage('toggleConsume')
          break;
        case 'F5':
          this.sendMessage('toggleCollisionMeshes')
          break;
        case 'F6':
          this.sendMessage('quitout')
          break;
        case 'F7':
          this.sendMessage('savePosition')
          break;
        case 'F8':
          this.sendMessage('toggleAI')
          break;
        case 'F9':
          this.sendMessage('toggleNoDamage')
          break;
        case 'F11':
          this.sendMessage('toggleStealth')
          break;

      }
    })

    ipcRenderer.on('flag-change', (evt, arg) => {
      this.$set(this.flags, arg.flag, arg.value)
    })
  }
}
</script>

<style lang='stylus'>
// http://paletton.com/#uid=13z0u0k++teOMZA+VHm+Xnv+xhu

body
  margin: 4px 0
  background: #222
  background-image: url('../public/sidekiro.png')
  background-size: cover

#app
  text-align: center;
  color: #2c3e50;
  display: flex;
  flex-direction: column;
  height: calc(100vh - 8px);

button
  cursor: pointer
  flex-grow: 1
  background-color: rgba(42, 42, 42, .8)
  color: white
  font-size: .8rem
  font-family: 'PT Sans', sans-serif
  margin: 4px 8px
  border: 1px solid #77aa99
  transition: background-color .25s ease
  &:hover {
    background-color: #77aa99
  }
  & *
    cursor: pointer

/*body
  background-color: #002F5A
  background-image: url('../public/sidega.png')
  background-size: cover
  font-family: 'PT Sans', sans-serif
  font-size: 1.25rem

button
  background: rgb(226, 226, 226)
  color: #222
  border: none
  border-radius: .25rem
  display: inline-block
  padding: .33rem .75rem
  margin: .125rem
  font-size: .9rem
  font-weight: 500
  cursor: pointer
  box-shadow: 3px 3px 3px*/

input[type='checkbox']
  height: 10px
  vertical-align: middle

</style>
