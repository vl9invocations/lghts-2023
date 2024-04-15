import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
import { appWindow } from '@tauri-apps/api/window'

createApp(App).mount("#app");

const btn1 = document.getElementById('titlebar-close');
btn1.addEventListener('click', () => appWindow.close());