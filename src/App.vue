<template lang='pug'>
div#app
  button(@click='toggleCollisionMeshes()') Toggle collision meshes (F5)
  button(@click='savePosition()') Save position (F7)
  button(@click='loadPosition()') Load position (F1)
  button Toggle AI
  button Toggle No Damage
  button Toggle Stealth
</template>

<script>
const ipcRenderer = window.ipcRenderer

export default {
  name: 'app',
  methods: {
    toggleCollisionMeshes () {
      ipcRenderer.send('execute', 'toggleCollisionMeshes')
    },
    savePosition () {
      ipcRenderer.send('execute', 'savePosition')
    },
    loadPosition () {
      ipcRenderer.send('execute', 'loadPosition')
    }
  },
  mounted () {
    document.body.addEventListener('keyup', evt => {
      switch (evt.code) {
        case 'F1':
          this.loadPosition()
          break;
        case 'F5':
          this.toggleCollisionMeshes()
          break;
        case 'F7':
          this.savePosition()
          break;
      }
    })
  }
}
</script>

<style lang='stylus'>
// http://paletton.com/#uid=13z0u0k++teOMZA+VHm+Xnv+xhu
#app 
  text-align: center;
  color: #2c3e50;
  display: flex;
  flex-direction: column

body
  background-color: #002F5A
  background-image: url('../public/sidega.png')
  background-size: cover
  font-family: 'PT Sans', sans-serif
  font-size: 1.25rem

button
  background: rgba(0, 80, 151, .8)
  color: white
  border: none
  border-radius: .5rem
  display: inline-block
  padding: .5rem 1rem
  margin: .5rem
  cursor: pointer

</style>
