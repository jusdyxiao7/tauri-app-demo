<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { WebviewWindow, appWindow } from "@tauri-apps/api/window";

// import { WebviewWindow } from "@tauri-apps/api/window"; //引入打开窗口功能

const greetMsg = ref("");
const name = ref("");

const open_out = () => {
  // 创建一个窗口
  const studentLoginWebView = new WebviewWindow('toStudentLogin', {
    url: 'http://117.132.0.103:17743/#/loginStudent',
    title: '搜救任务协调',
    width: 500,
    height: 400,
    x: 100,
    y: 200
  })
  studentLoginWebView.once("tauri://created", () => {
    alert('搜救任务协调窗口创建')
  })
  studentLoginWebView.once("tauri://error", (error) => {
    console.log(error);
    alert(error)
  })
}



async function greet() {
  // appWindow.hide()
  open_out()

  // invoke('open_toStudentLogin')

  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", { name: name.value });
}
</script>

<template>
  <form class="row" @submit.prevent="greet">
    <input id="greet-input" v-model="name" placeholder="Enter a name..." />
    <button type="submit">Greet</button>
  </form>

  <p>{{ greetMsg }}</p>
</template>
