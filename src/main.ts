import { createApp } from 'vue'
import App from './App.vue'
import 'virtual:uno.css'
import router from './router';
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import './styles.css'
import { getConfig, saveConfig } from './utils/config';

saveConfig(getConfig());

const app = createApp(App)
app.use(router)
app.use(ElementPlus)
app.mount('#app')