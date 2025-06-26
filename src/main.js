import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'
const appWindow = getCurrentWebviewWindow()

createApp(App).mount("#app");

const btn1 = document.getElementById('titlebar-close');
btn1.addEventListener('click', () => appWindow.close());