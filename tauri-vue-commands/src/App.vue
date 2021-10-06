<template>
  <img alt="Vue logo" src="./assets/logo.png">
  <button v-on:click="reverseMessage">无参数</button>
    <button v-on:click="reverseMessage2">有参数的</button>
    <button v-on:click="reverseMessage3">返回数据</button>
     <button v-on:click="reverseMessageW">访问windows窗口</button>
     <button v-on:click="appHandle">AppHandle</button>
  <HelloWorld msg="Welcome to Your Vue.js App"/>
</template>

<script>
import HelloWorld from './components/HelloWorld.vue'
import { tauri } from '@tauri-apps/api/'
import { invoke } from '@tauri-apps/api/tauri'

console.log(tauri.invoke==invoke?"可以使用":"不能使用");

export default {
  name: 'App',
  components: {
    HelloWorld
  },
  methods: {
          reverseMessage: function() {
             const invoke = window.__TAURI__.invoke
             invoke('my_custom_command');
          },
          reverseMessage2: function() {
             const invoke = window.__TAURI__.invoke
             invoke('my_custom_command2', { invokeMessage: 'Hello!' })
          },
          reverseMessage3: function() {
             const invoke = window.__TAURI__.invoke

             //invoke('my_custom_command').then((message) => console.log(message))
             
             invoke('my_custom_command3').then(
               function(message){
                  alert("message");
                  console.log(message);
                  
              });
          },
          reverseMessageW:function(){
            const invoke = window.__TAURI__.invoke;
            invoke('my_custom_command_w');
          },
          appHandle:function(){
            const invoke = window.__TAURI__.invoke;
            invoke('my_custom_command_app');
          }
  }
}
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
