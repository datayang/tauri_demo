import { createApp } from 'vue'

// import { Button, Select } from 'element-ui';

import App from './App.vue'
import installElementPlus from './plugins/element'

// createApp.component(Button.name, Button);
// createApp.component(Select.name, Select);

const app = createApp(App)
installElementPlus(app)
app.mount('#app')